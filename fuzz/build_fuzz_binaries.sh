#!/bin/bash
set -e

cd "$(dirname "$0")/.." 

FLAGS="-include test/fuzz/fuzz-config.h -Ireference-c -O0 -DDEBUG -g"

OUTDIR=".build/differential-fuzz"
mkdir -p "$OUTDIR"

echo "== Building fuzz-c: original C source + unmodified fuzz.c =="
C_SRCS=$(find reference-c/ -type f -name '*.c')
cc $FLAGS -DMPACK_EXTENSIONS=1 $C_SRCS test/fuzz/fuzz.c -o "$OUTDIR/fuzz-c"
echo "  -> $OUTDIR/fuzz-c"

echo "== Building fuzz-rust: same fuzz.c, linked against Rust staticlib =="

TEST_FLAGS="-Ireference-c -Itest/unit/src -DMPACK_HAS_CONFIG=1 -DMPACK_STDLIB=1 -DMPACK_STDIO=1 \
-DMPACK_MALLOC=test_malloc -DMPACK_FREE=test_free -std=c11 -O0 -DDEBUG -g"

cc $TEST_FLAGS -c test/unit/src/test-system.c -o "$OUTDIR/test-system.o"
cc -Ireference-c -O0 -DDEBUG -g -c test/fuzz/fuzz_test_shim.c -o "$OUTDIR/fuzz_test_shim.o"
cc $FLAGS -DMPACK_EXTENSIONS=1 -c test/fuzz/fuzz.c -o "$OUTDIR/fuzz.o"

cc -g "$OUTDIR/fuzz.o" "$OUTDIR/test-system.o" "$OUTDIR/fuzz_test_shim.o" \
   -L target/debug -lmpack_rs \
   -lpthread -ldl -lm \
   -o "$OUTDIR/fuzz-rust"
echo "  -> $OUTDIR/fuzz-rust"

echo "== Building fuzz-expect-c: original C source + fuzz_expect.c =="
cc $FLAGS -DMPACK_EXTENSIONS=1 $C_SRCS test/fuzz/fuzz_expect.c -o "$OUTDIR/fuzz-expect-c"
echo "  -> $OUTDIR/fuzz-expect-c"

echo "== Building fuzz-expect-rust: fuzz_expect.c, linked against Rust staticlib =="
cc $FLAGS -DMPACK_EXTENSIONS=1 -c test/fuzz/fuzz_expect.c -o "$OUTDIR/fuzz_expect.o"
cc -g "$OUTDIR/fuzz_expect.o" "$OUTDIR/test-system.o" "$OUTDIR/fuzz_test_shim.o" \
   -L target/debug -lmpack_rs \
   -lpthread -ldl -lm \
   -o "$OUTDIR/fuzz-expect-rust"
echo "  -> $OUTDIR/fuzz-expect-rust"

echo "== Done =="
ls -la "$OUTDIR"
