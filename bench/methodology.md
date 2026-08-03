# Benchmark Methodology

## What we measured

We reused the already-built `fuzz-c` and `fuzz-rust` binaries from
`.build/differential-fuzz/` — the same two binaries used for differential
fuzzing. Both are built from the identical driver source (`test/fuzz/fuzz.c`),
linked against different backends (the real C library vs. our Rust
translation). This means the benchmark measures the same code path already
verified for behavioral equivalence, not a synthetic or isolated
microbenchmark — the comparison is apples-to-apples by construction.

**Honest scope:** this measures the driver's full pipeline per invocation
(reader → writer → tree-parse), including process startup/teardown for each
run, not raw in-process library call latency. We measured it this way
because it's what our existing, trusted verification infrastructure gives
us directly, and process-level timing includes real-world overhead
(process spawn, allocator init) that a pure in-process microbenchmark would
hide. If time allows a follow-up, an in-process benchmark (calling the
library directly in a loop within one process) would isolate pure
library-code performance from process overhead.

## Metrics

- **Startup time**: wall-clock time to run each binary on an empty input,
  averaged over 200 repetitions. Isolates process/init overhead from actual
  parsing work.
- **Per-input latency (p50/p99) and throughput**: wall-clock time to
  process each file in the real corpus (`test/messagepack/*.mp`), repeated
  100 times per file, timed with `time.perf_counter()` around each
  subprocess invocation. p99 is reported (not just mean/throughput)
  specifically because a single average hides tail latency spikes that
  matter for a parsing library.
- **Peak memory (RSS)**: intended to be measured via `/usr/bin/time -v`
  on a single run against the largest corpus file, reading "Maximum
  resident set size" from its output; reported as `null` if unavailable.
  See **Status** below — in this submission's build image, `time -v`
  is not present, so RSS is `null` for both binaries, not measured and
  found equal.

## How to reproduce

```sh
# from the project root, after rust/build_fuzz_binaries.sh has been run
python3 bench/bench.py
```

Writes `bench/results.json` and prints a live summary. No arguments needed.

## Known limitations

- Process-spawn overhead is included in every timing, which will make both
  binaries' absolute numbers look slower than pure in-library performance —
  but since this overhead applies equally to both binaries built from the
  same driver, the *relative* comparison (C vs. Rust) is still fair.
- We did not pin CPU frequency scaling or isolate the benchmark to a
  dedicated core; numbers may have some run-to-run variance from other
  system load. Given hackathon time constraints we prioritized getting a
  real, honest measurement over a fully isolated one — this is disclosed
  rather than hidden.
- When `results.json` is generated, note the machine spec (CPU, OS) it
  was measured on directly in this file, next to wherever the numbers
  are summarized, so the numbers are never separated from the hardware
  they came from.

## Status

Run and complete. Full numbers in `bench/results.json`; summary below.

**Machine:** Intel Core i5-11300H @ 3.10GHz, `Linux arih
5.15.167.4-microsoft-standard-WSL2 x86_64` (Docker Desktop / WSL2 host).
Benchmark itself ran inside the `rustlang/rust:nightly-bookworm`
container built from this repo's `Dockerfile` (see `Dockerfile` for the
build image; the numbers below reflect that container's environment,
not bare-metal WSL2).

| Metric | C (original) | Rust (port) |
|---|---:|---:|
| Startup p50 | 1.708 ms | 1.706 ms |
| Startup p99 | 2.156 ms | 2.160 ms |
| Latency p50 | 1.746 ms | 1.745 ms |
| Latency p99 | 2.256 ms | 2.293 ms |
| Latency max | 2.525 ms | 5.239 ms |
| Throughput | 576.0 runs/sec | 567.1 runs/sec |
| Peak RSS | unavailable | unavailable |

500 total runs each (100 reps × 5 corpus files), 234,300 total input
bytes processed per binary. Startup and p50 latency are within noise of
each other between C and Rust. Rust's p99 and max latency are somewhat
higher than C's (2.293ms vs 2.256ms p99; 5.239ms vs 2.525ms max) — the
max in particular is a ~2x outlier worth a follow-up if there's time
(e.g. rerun with more reps to see if it's a one-off scheduling blip or
a repeatable tail-latency difference, possibly related to allocator
behavior in the Rust build). We're reporting it rather than omitting it.

**Peak RSS is `null` for both binaries, not because memory usage is
equal** — it's unmeasured. We confirmed directly inside the running
container:

```sh
$ /usr/bin/time -v echo test
OCI runtime exec failed: exec failed: unable to start container
process: exec: "/usr/bin/time": stat /usr/bin/time: no such file or directory
```

`time -v` (the Debian `time` package) isn't installed in
`rustlang/rust:nightly-bookworm`, and `build-essential` doesn't pull it
in as a dependency. This is a build-image gap, not a code bug — fixing
it means adding `time` to the `apt-get install` list in `Dockerfile`
and re-running. We're flagging this explicitly rather than leaving
`null` unexplained, per the "numbers are honest" rule: unmeasured is
not the same as "measured and found equal," and shouldn't be presented
as such.

To reproduce (with the `time` fix applied):

```sh
# in Dockerfile, add `time \` to the apt-get install list, then:
docker build -t mpack-port .
docker run --rm -it -v "$(pwd)":/repo --entrypoint bash mpack-port
bash fuzz/build_fuzz_binaries.sh
python3 bench/bench.py
```

Using `-v "$(pwd)":/repo` (rather than a bare `docker run --rm`) mounts
the host project directory into the container, so `results.json` lands
directly on disk instead of being lost when the container exits.