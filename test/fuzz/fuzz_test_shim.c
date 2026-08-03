/*
 * fuzz_test_shim.c
 *
 * The Rust crate (rust/) was transpiled with MPACK_MALLOC=test_malloc,
 * MPACK_FREE=test_free, and MPACK_CUSTOM_ASSERT=1 (same config as the unit
 * test suite), so it links against test_true_impl/tests/passes (used by the
 * TEST_TRUE macro inside test-system.c's test_malloc/test_free/test_fopen/etc.)
 * and against mpack_assert_fail/mpack_break_hit.
 *
 * Those symbols normally live in test/unit/src/test.c, but that file also
 * defines main(), which would collide with fuzz.c's main(). This shim
 * provides equivalent behavior (mirroring test.c's test_true_impl and its
 * assert/break handlers) without a competing main(), so it's a differential-
 * fuzz-only helper, not a modification of the original test suite.
 *
 * Behavior mirrors test.c: on any assert or break, print and abort so the
 * fuzzer sees a real crash. Neither fuzz-c nor fuzz-rust prime test_jmp_set /
 * test_break_set (those are only set by specific unit tests exercising
 * TEST_ASSERT/TEST_BREAK), so this always takes the abort path — consistent
 * with test.c's own fallback when unprimed.
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <stdbool.h>

int passes;
int tests;

void test_true_impl(bool result, const char* file, int line, const char* format, ...) {
    ++tests;
    if (result) {
        ++passes;
    } else {
        printf("TEST FAILED AT %s:%i --", file, line);
        va_list args;
        va_start(args, format);
        vprintf(format, args);
        va_end(args);
        printf("\n");
        abort();
    }
}

void mpack_assert_fail(const char* message) {
    printf("assertion hit! %s\n", message);
    abort();
}

void mpack_break_hit(const char* message) {
    printf("break hit! %s\n", message);
    abort();
}
