#!/usr/bin/env python3
"""
Benchmark driver for mpack C vs Rust port.

Reuses the already-built .build/differential-fuzz/fuzz-c and fuzz-rust
binaries (same fuzz.c driver, different backend) -- the same binaries used
for differential fuzzing -- so this is an apples-to-apples comparison of
the same code path, not a synthetic microbenchmark.

Measures, for each binary:
  - startup time: wall-clock time to process a minimal (empty) input,
    averaged over many runs -- isolates process/init overhead
  - per-input latency distribution (p50/p99) and throughput: wall-clock
    time to process each real corpus file, repeated many times
  - peak memory (RSS): via `/usr/bin/time -v`, run separately since it
    changes process overhead and shouldn't be mixed into latency timings

Writes bench/results.json and prints a human-readable summary.
Run from the project root: python3 bench/bench.py
"""

import json
import os
import statistics
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FUZZ_C = os.path.join(ROOT, ".build", "differential-fuzz", "fuzz-c")
FUZZ_RUST = os.path.join(ROOT, ".build", "differential-fuzz", "fuzz-rust")
CORPUS_DIR = os.path.join(ROOT, "test", "messagepack")

STARTUP_REPS = 200
LATENCY_REPS_PER_FILE = 100


def check_binaries():
    for path in (FUZZ_C, FUZZ_RUST):
        if not os.path.isfile(path):
            print(f"ERROR: {path} not found. Run rust/build_fuzz_binaries.sh first.")
            sys.exit(1)


def load_corpus():
    files = []
    if os.path.isdir(CORPUS_DIR):
        for name in sorted(os.listdir(CORPUS_DIR)):
            if name.endswith(".mp"):
                path = os.path.join(CORPUS_DIR, name)
                with open(path, "rb") as f:
                    files.append((name, f.read()))
    if not files:
        print(f"ERROR: no .mp corpus files found under {CORPUS_DIR}")
        sys.exit(1)
    return files


def timed_run(path, data):
    start = time.perf_counter()
    subprocess.run([path], input=data, stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL, timeout=10)
    return time.perf_counter() - start


def measure_startup(path):
    times = [timed_run(path, b"") for _ in range(STARTUP_REPS)]
    return {
        "reps": STARTUP_REPS,
        "mean_ms": statistics.mean(times) * 1000,
        "p50_ms": statistics.median(times) * 1000,
        "p99_ms": sorted(times)[int(len(times) * 0.99) - 1] * 1000,
    }


def measure_latency_and_throughput(path, corpus):
    all_times = []
    total_bytes = 0
    wall_start = time.perf_counter()
    for _name, data in corpus:
        for _ in range(LATENCY_REPS_PER_FILE):
            all_times.append(timed_run(path, data))
            total_bytes += len(data)
    wall_elapsed = time.perf_counter() - wall_start
    n = len(all_times)
    return {
        "total_runs": n,
        "total_input_bytes": total_bytes,
        "mean_ms": statistics.mean(all_times) * 1000,
        "p50_ms": statistics.median(all_times) * 1000,
        "p99_ms": sorted(all_times)[int(n * 0.99) - 1] * 1000,
        "max_ms": max(all_times) * 1000,
        "throughput_runs_per_sec": n / wall_elapsed,
        "wall_elapsed_sec": wall_elapsed,
    }


def measure_peak_rss(path, data):
    """Peak resident set size in KB, via /usr/bin/time -v. Linux only."""
    try:
        result = subprocess.run(
            ["/usr/bin/time", "-v", path],
            input=data, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            timeout=10,
        )
        for line in result.stderr.decode(errors="replace").splitlines():
            if "Maximum resident set size" in line:
                return int(line.strip().split()[-1])
    except FileNotFoundError:
        return None
    return None


def main():
    check_binaries()
    corpus = load_corpus()
    print(f"Loaded {len(corpus)} corpus files from {CORPUS_DIR}")

    results = {"corpus_files": [name for name, _ in corpus]}

    for label, path in (("c", FUZZ_C), ("rust", FUZZ_RUST)):
        print(f"\n== Benchmarking {label} ({path}) ==")
        print(f"  startup ({STARTUP_REPS} reps on empty input)...")
        startup = measure_startup(path)
        print(f"  latency/throughput ({LATENCY_REPS_PER_FILE} reps per corpus file)...")
        latency = measure_latency_and_throughput(path, corpus)
        print("  peak RSS (single run on largest corpus file)...")
        largest = max(corpus, key=lambda kv: len(kv[1]))
        peak_rss_kb = measure_peak_rss(path, largest[1])

        results[label] = {
            "startup": startup,
            "latency_throughput": latency,
            "peak_rss_kb": peak_rss_kb,
        }
        print(f"  startup p50={startup['p50_ms']:.3f}ms p99={startup['p99_ms']:.3f}ms")
        print(f"  latency p50={latency['p50_ms']:.3f}ms p99={latency['p99_ms']:.3f}ms "
              f"throughput={latency['throughput_runs_per_sec']:.1f} runs/sec")
        print(f"  peak RSS={peak_rss_kb} KB" if peak_rss_kb else "  peak RSS=unavailable")

    out_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "results.json")
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nWrote {out_path}")


if __name__ == "__main__":
    main()
