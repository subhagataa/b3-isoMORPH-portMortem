#!/usr/bin/env python3
import subprocess, sys

FUZZ_C = ".build/differential-fuzz/fuzz-c"
FUZZ_RUST = ".build/differential-fuzz/fuzz-rust"

def run(path, data):
    p = subprocess.run([path], input=data, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=5)
    return p.returncode, p.stdout

def diverges(data):
    c = run(FUZZ_C, data)
    r = run(FUZZ_RUST, data)
    return c != r

def shrink(data):
    changed = True
    while changed:
        changed = False
        for chunk in (64, 16, 4, 1):
            i = len(data)
            while i > 0:
                candidate = data[:max(0, i - chunk)]
                if candidate and diverges(candidate):
                    data = candidate
                    changed = True
                    i = len(data)
                else:
                    i -= chunk
        i = 0
        while i < len(data):
            candidate = data[:i] + data[i+1:]
            if candidate and diverges(candidate):
                data = candidate
                changed = True
            else:
                i += 1
    return data

if __name__ == "__main__":
    path = sys.argv[1]
    data = open(path, "rb").read()
    assert diverges(data), "input does not reproduce a divergence"
    print(f"original: {len(data)} bytes")
    shrunk = shrink(data)
    print(f"shrunk to: {len(shrunk)} bytes")
    print(shrunk.hex())
    open(path + ".shrunk", "wb").write(shrunk)
