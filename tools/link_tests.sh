#!/bin/bash
set -e

cd "$(dirname "$0")/.."   

FLAGS="-Wall -Wextra -Werror -Wconversion -Wundef -Wshadow -Wcast-qual -g -Ireference-c -Itest/unit/src \
-DMPACK_VARIANT_BUILDS=1 -DMPACK_HAS_CONFIG=1 -fdiagnostics-color=always -Wstrict-aliasing=3 -Wpedantic \
-Wfloat-conversion -fstrict-aliasing -DMPACK_READER=1 -DMPACK_WRITER=1 -DMPACK_EXPECT=1 -DMPACK_NODE=1 \
-DMPACK_COMPATIBILITY=1 -DMPACK_EXTENSIONS=1 -DMPACK_STDLIB=1 -DMPACK_MALLOC=test_malloc -DMPACK_FREE=test_free \
-DMPACK_STDIO=1 -std=c11 -Wc++-compat -Wmissing-prototypes -Wstrict-prototypes -Og -DDEBUG"

OUTDIR=".build/rust-link"
mkdir -p "$OUTDIR"

echo "== Compiling original test suite .c files (unmodified) =="
TEST_SRCS="test-system test-expect test-node test-buffer test-file test-write test test-common test-builder test-reader"
OBJS=""
for name in $TEST_SRCS; do
    src="test/unit/src/${name}.c"
    obj="$OUTDIR/${name}.o"
    echo "  compiling $src"
    cc $FLAGS -c "$src" -o "$obj"
    OBJS="$OBJS $obj"
done

echo "== Linking against Rust staticlib (librust_mpack) instead of original mpack .c files =="
cc -g $OBJS \
   -L target/debug -lmpack_rs \
   -lpthread -ldl -lm \
   -o "$OUTDIR/runner"

echo "== Running tests =="
"$OUTDIR/runner"
