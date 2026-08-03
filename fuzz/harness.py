#!/usr/bin/env python3
"""
Differential fuzzing driver for mpack C vs Rust port.

Feeds the same input bytes to both .build/differential-fuzz/fuzz-c and
fuzz-rust, compares stdout + exit code, and logs any divergence with a hex
dump of the input for reproducibility.

Usage:
    python3 diff_fuzz.py [--iterations N] [--seed N]

Corpus:
    Seeds from test/messagepack/*.mp (real valid/edge-case msgpack data),
    then generates mutated variants (byte flips / insert / delete / truncate)
    plus a smaller share of pure-random inputs, to cover both "deep, mostly-
    valid" and "shallow, garbage" input space.
"""

import argparse
import os
import random
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) if __file__.endswith(
    "rust/diff_fuzz.py"
) else os.path.dirname(os.path.abspath(__file__))

# Resolve project root robustly: this script may be run from repo root or rust/.
def find_project_root():
    here = os.path.abspath(os.path.dirname(__file__))
    for candidate in [here, os.path.dirname(here)]:
        if os.path.isdir(os.path.join(candidate, "test", "messagepack")):
            return candidate
    return here

PROJECT_ROOT = find_project_root()
FUZZ_C = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "fuzz-c")
FUZZ_RUST = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "fuzz-rust")
CORPUS_DIR = os.path.join(PROJECT_ROOT, "test", "messagepack")
LOG_DIR = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "findings")
TIMEOUT_SECONDS = 5


def load_corpus():
    files = []
    if os.path.isdir(CORPUS_DIR):
        for name in sorted(os.listdir(CORPUS_DIR)):
            if name.endswith(".mp"):
                path = os.path.join(CORPUS_DIR, name)
                with open(path, "rb") as f:
                    files.append((name, f.read()))
    return files


def mutate(data: bytes, rng: random.Random) -> bytes:
    """Apply one or more small random mutations to a corpus seed."""
    if not data:
        return bytes(rng.randrange(256) for _ in range(rng.randint(1, 16)))

    b = bytearray(data)
    n_mutations = rng.randint(1, 4)
    for _ in range(n_mutations):
        op = rng.choice(["flip", "insert", "delete", "truncate"])
        if op == "flip" and b:
            idx = rng.randrange(len(b))
            b[idx] = rng.randrange(256)
        elif op == "insert":
            idx = rng.randrange(len(b) + 1)
            b.insert(idx, rng.randrange(256))
        elif op == "delete" and len(b) > 1:
            idx = rng.randrange(len(b))
            del b[idx]
        elif op == "truncate" and len(b) > 1:
            cut = rng.randint(1, len(b) - 1)
            b = b[:cut]
    return bytes(b)


def random_bytes(rng: random.Random) -> bytes:
    length = rng.randint(0, 256)
    return bytes(rng.randrange(256) for _ in range(length))


def gen_input(rng: random.Random, corpus):
    """~70% mutated corpus, ~30% pure random."""
    if corpus and rng.random() < 0.7:
        name, data = rng.choice(corpus)
        return mutate(data, rng), f"mutated:{name}"
    else:
        return random_bytes(rng), "random"


def run_binary(path, data):
    # abort() does not flush stdio buffers, and C stdio is fully-buffered
    # (not line-buffered) when stdout isn't a terminal -- which is always
    # true here since we pipe it. Without forcing unbuffered output, any
    # message printed right before a crash is silently lost. `stdbuf -o0 -e0`
    # forces unbuffered stdout/stderr on the child process.
    try:
        proc = subprocess.run(
            ["stdbuf", "-o0", "-e0", path],
            input=data,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=TIMEOUT_SECONDS,
        )
        return proc.returncode, proc.stdout
    except subprocess.TimeoutExpired:
        return "TIMEOUT", b""


def hexdump(data: bytes) -> str:
    lines = []
    for i in range(0, len(data), 16):
        chunk = data[i:i + 16]
        hex_part = " ".join(f"{b:02x}" for b in chunk)
        ascii_part = "".join(chr(b) if 32 <= b < 127 else "." for b in chunk)
        lines.append(f"{i:08x}: {hex_part:<47} {ascii_part}")
    return "\n".join(lines) if lines else "(empty input)"


def log_finding(idx, data, source, c_result, r_result):
    os.makedirs(LOG_DIR, exist_ok=True)
    c_exit, c_out = c_result
    r_exit, r_out = r_result
    finding_path = os.path.join(LOG_DIR, f"finding_{idx:06d}.txt")
    raw_path = os.path.join(LOG_DIR, f"finding_{idx:06d}.bin")

    with open(raw_path, "wb") as f:
        f.write(data)

    with open(finding_path, "w") as f:
        f.write(f"Source: {source}\n")
        f.write(f"Raw input saved to: {os.path.basename(raw_path)}\n")
        f.write(f"Input length: {len(data)} bytes\n\n")
        f.write(f"fuzz-c   exit: {c_exit}\n")
        f.write(f"fuzz-rust exit: {r_exit}\n\n")
        f.write("--- fuzz-c output ---\n")
        f.write(c_out.decode("utf-8", errors="replace"))
        f.write("\n\n--- fuzz-rust output ---\n")
        f.write(r_out.decode("utf-8", errors="replace"))
        f.write("\n\n--- hex dump ---\n")
        f.write(hexdump(data))
        f.write("\n")

    return finding_path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--iterations", type=int, default=5000)
    parser.add_argument("--seed", type=int, default=None)
    parser.add_argument("--target", choices=["reader", "expect"], default="reader")
    args = parser.parse_args()

    global FUZZ_C, FUZZ_RUST, LOG_DIR
    if args.target == "expect":
        FUZZ_C = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "fuzz-expect-c")
        FUZZ_RUST = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "fuzz-expect-rust")
        LOG_DIR = os.path.join(PROJECT_ROOT, ".build", "differential-fuzz", "findings-expect")

    if not os.path.isfile(FUZZ_C):
        print(f"ERROR: {FUZZ_C} not found. Run rust/build_fuzz_binaries.sh first.")
        sys.exit(1)
    if not os.path.isfile(FUZZ_RUST):
        print(f"ERROR: {FUZZ_RUST} not found. Run rust/build_fuzz_binaries.sh first.")
        sys.exit(1)

    corpus = load_corpus()
    print(f"Loaded {len(corpus)} corpus seed files from {CORPUS_DIR}")

    rng = random.Random(args.seed)
    n_mismatches = 0
    n_timeouts = 0
    start = time.time()

    for i in range(1, args.iterations + 1):
        data, source = gen_input(rng, corpus)
        if args.target == "expect":
            selector = bytes([rng.randint(0, 255)])
            data = selector + data

        c_result = run_binary(FUZZ_C, data)
        r_result = run_binary(FUZZ_RUST, data)

        c_exit, c_out = c_result
        r_exit, r_out = r_result

        if c_exit == "TIMEOUT" or r_exit == "TIMEOUT":
            n_timeouts += 1

        if c_exit != r_exit or c_out != r_out:
            n_mismatches += 1
            path = log_finding(n_mismatches, data, source, c_result, r_result)
            print(f"[{i}/{args.iterations}] MISMATCH #{n_mismatches} "
                  f"(c_exit={c_exit} r_exit={r_exit}) -> {path}")

        if i % 500 == 0:
            elapsed = time.time() - start
            print(f"[{i}/{args.iterations}] progress ({elapsed:.1f}s elapsed, "
                  f"{n_mismatches} mismatches, {n_timeouts} timeouts so far)")

    elapsed = time.time() - start
    print("\n=== Done ===")
    print(f"Ran {args.iterations} inputs in {elapsed:.1f}s")
    print(f"Mismatches found: {n_mismatches}")
    print(f"Timeouts: {n_timeouts}")
    if n_mismatches:
        print(f"Findings logged under: {LOG_DIR}")


if __name__ == "__main__":
    main()
