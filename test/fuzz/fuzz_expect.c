#ifdef MPACK_FUZZ
#include "mpack/mpack.h"
#ifndef MPACK_FUZZ_CONFIG_H
#error "This should be built with fuzz-config.h as a prefix header."
#endif

// Fuzz harness for mpack_expect_* functions. Unlike fuzz.c (which reads
// whatever tag comes next via mpack_read_tag), mpack_expect_* functions
// each demand a SPECIFIC type and flag an error if the reader's next
// value doesn't match. We use the first byte of input as a selector to
// pick which mpack_expect_* function to exercise, then feed the rest of
// the input to the reader for that function to consume.
//
// The goal is simply to confirm the C and Rust implementations behave
// identically (same accept/reject decision, same value, same error state)
// for the same (selector, remaining_bytes) pair -- differential fuzzing,
// same approach as fuzz.c, just targeting mpack_expect instead of
// mpack_read_tag/mpack_write_tag/mpack_tree_parse.

static void print_error_state(mpack_reader_t* reader) {
    mpack_error_t err = mpack_reader_error(reader);
    if (err != mpack_ok) {
        fprintf(stderr, "expect error: %d\n", (int)err);
    }
}

int main(int argc, char** argv) {
    size_t input_capacity = 65536;
    size_t input_size = 0;
    char* input_data = (char*)malloc(input_capacity);
    if (input_data == NULL) {
        fprintf(stderr, "out of memory reading stdin!\n");
        return EXIT_FAILURE;
    }
    for (;;) {
        if (input_size == input_capacity) {
            input_capacity *= 2;
            char* grown = (char*)realloc(input_data, input_capacity);
            if (grown == NULL) {
                free(input_data);
                fprintf(stderr, "out of memory reading stdin!\n");
                return EXIT_FAILURE;
            }
            input_data = grown;
        }
        size_t got = fread(input_data + input_size, 1, input_capacity - input_size, stdin);
        input_size += got;
        if (got == 0)
            break;
    }

    if (input_size < 1) {
        fprintf(stderr, "input too short (need at least 1 selector byte)!\n");
        free(input_data);
        return EXIT_FAILURE;
    }

    uint8_t selector = (uint8_t)input_data[0];
    char* rest = input_data + 1;
    size_t rest_size = input_size - 1;

    mpack_reader_t reader;
    mpack_reader_init_data(&reader, rest, rest_size);

    // 12 selector cases, matching the non-inline mpack_expect_* variants
    // declared in src/mpack/mpack-expect.h (the MPACK_INLINE *_max/*_range
    // convenience wrappers just call these, so exercising these covers
    // the same underlying logic).
    switch (selector % 12) {
        case 0: {
            uint8_t v = mpack_expect_u8(&reader);
            fprintf(stderr, "u8: %u\n", (unsigned)v);
            break;
        }
        case 1: {
            uint16_t v = mpack_expect_u16(&reader);
            fprintf(stderr, "u16: %u\n", (unsigned)v);
            break;
        }
        case 2: {
            uint32_t v = mpack_expect_u32(&reader);
            fprintf(stderr, "u32: %u\n", (unsigned)v);
            break;
        }
        case 3: {
            uint64_t v = mpack_expect_u64(&reader);
            fprintf(stderr, "u64: %llu\n", (unsigned long long)v);
            break;
        }
        case 4: {
            int8_t v = mpack_expect_i8(&reader);
            fprintf(stderr, "i8: %d\n", (int)v);
            break;
        }
        case 5: {
            int16_t v = mpack_expect_i16(&reader);
            fprintf(stderr, "i16: %d\n", (int)v);
            break;
        }
        case 6: {
            int32_t v = mpack_expect_i32(&reader);
            fprintf(stderr, "i32: %d\n", (int)v);
            break;
        }
        case 7: {
            int64_t v = mpack_expect_i64(&reader);
            fprintf(stderr, "i64: %lld\n", (long long)v);
            break;
        }
        case 8: {
            float v = mpack_expect_float(&reader);
            fprintf(stderr, "float: %f\n", (double)v);
            break;
        }
        case 9: {
            double v = mpack_expect_double(&reader);
            fprintf(stderr, "double: %f\n", v);
            break;
        }
        case 10: {
            uint8_t v = mpack_expect_u8_max(&reader, 200);
            fprintf(stderr, "u8_max: %u\n", (unsigned)v);
            break;
        }
        case 11: {
            int32_t v = mpack_expect_i32_range(&reader, -1000, 1000);
            fprintf(stderr, "i32_range: %d\n", (int)v);
            break;
        }
    }

    print_error_state(&reader);

    if (mpack_reader_destroy(&reader) != mpack_ok) {
        fprintf(stderr, "error destroying reader!\n");
        free(input_data);
        return EXIT_FAILURE;
    }

    free(input_data);
    return EXIT_SUCCESS;
}
#endif
