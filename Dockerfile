# mpack: C -> Rust port
#
#   docker build -t mpack-port .
#   docker run --rm mpack-port
#
# Builds the Rust staticlib (src/, via the root Cargo.toml), then links it
# against the ORIGINAL, unmodified C test suite (test/unit/src) in place of
# the original C implementation, and runs the resulting binary. A clean
# run (exit 0, all checks passing) is the "it works" signal.

FROM rustlang/rust:nightly-bookworm AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /repo
COPY . .

# Build the Rust port (staticlib + rlib, per Cargo.toml)
RUN cargo build

# Link the Rust staticlib against the original, unmodified C test suite
# and run it. This is the ground-truth correctness check: same test
# assertions the original C implementation had to pass.
RUN chmod +x tools/link_tests.sh

ENTRYPOINT ["tools/link_tests.sh"]
