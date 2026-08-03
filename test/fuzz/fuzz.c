
#ifdef MPACK_FUZZ


#include "mpack/mpack.h"

#ifndef MPACK_FUZZ_CONFIG_H
#error "This should be built with fuzz-config.h as a prefix header."
#endif

static void print_callback(void* context, const char* data, size_t count) {
    fwrite(data, 1, count, stdout);
}

static void transfer_bytes(mpack_reader_t* reader, mpack_writer_t* writer, uint32_t count) {
    if (mpack_should_read_bytes_inplace(reader, count)) {
        const char* data = mpack_read_bytes_inplace(reader, count);
        if (mpack_reader_error(reader) == mpack_ok)
            mpack_write_bytes(writer, data, count);
        return;
    }

    while (count > 0) {
        char buffer[79];
        uint32_t step = (count < sizeof(buffer)) ? count : sizeof(buffer);
        mpack_read_bytes(reader, buffer, step);
        if (mpack_reader_error(reader) != mpack_ok)
            return;
        mpack_write_bytes(writer, buffer, step);
        count -= step;
    }
}

static void transfer_element(mpack_reader_t* reader, mpack_writer_t* writer, int depth) {

    // We apply a depth limit manually right now to avoid a stack overflow. A
    // depth limit should probably be added to the reader and tree at some
    // point because even though the reader and tree can themselves handle
    // arbitrary depths, any dynamic use that doesn't account for this is
    // likely to be vulnerable to such stack overflows.
    if (depth >= 1024) {
        fprintf(stderr, "hit depth limit!\n");
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return;
    }
    ++depth;

    mpack_tag_t tag = mpack_read_tag(reader);
    if (mpack_reader_error(reader) != mpack_ok) {
        fprintf(stderr, "error reading tag!\n");
        return;
    }

    /*
    static char describe_buffer[64];
    mpack_tag_debug_describe(tag, describe_buffer, sizeof(describe_buffer));
    printf("%s\n", describe_buffer);
    */

    mpack_write_tag(writer, tag);

    switch (tag.type) {
        #if MPACK_EXTENSIONS
        case mpack_type_ext: // fallthrough
        #endif
        case mpack_type_str: // fallthrough
        case mpack_type_bin:
            transfer_bytes(reader, writer, mpack_tag_bytes(&tag));
            if (mpack_reader_error(reader) != mpack_ok)
                return;
            mpack_done_type(reader, tag.type);
            mpack_finish_type(writer, tag.type);
            break;

        case mpack_type_map:
            for (uint32_t i = 0; i < mpack_tag_map_count(&tag); ++i) {
                transfer_element(reader, writer, depth);
                if (mpack_reader_error(reader) != mpack_ok)
                    return;
                transfer_element(reader, writer, depth);
                if (mpack_reader_error(reader) != mpack_ok)
                    return;
            }
            mpack_done_map(reader);
            mpack_finish_map(writer);
            break;

        case mpack_type_array:
            for (uint32_t i = 0; i < mpack_tag_array_count(&tag); ++i) {
                transfer_element(reader, writer, depth);
                if (mpack_reader_error(reader) != mpack_ok)
                    return;
            }
            mpack_done_array(reader);
            mpack_finish_array(writer);
            break;

        default:
            break;
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

    char* data;
    size_t size;
    mpack_writer_t writer;
    mpack_writer_init_growable(&writer, &data, &size);

    mpack_reader_t reader;
    mpack_reader_init_data(&reader, input_data, input_size);

    transfer_element(&reader, &writer, 0);

    if (mpack_reader_destroy(&reader) != mpack_ok || mpack_writer_destroy(&writer) != mpack_ok) {
        fprintf(stderr, "error in reader or writer!\n");
        free(input_data);
        return EXIT_FAILURE;
    }

    mpack_tree_t tree;
    mpack_tree_init_data(&tree, input_data, input_size);
    mpack_tree_parse(&tree);
    if (mpack_tree_error(&tree) != mpack_ok) {
        fprintf(stderr, "error parsing tree!\n");
        mpack_tree_destroy(&tree);
        free(input_data);
        return EXIT_FAILURE;
    }

    mpack_node_print_to_callback(mpack_tree_root(&tree), print_callback, NULL);

    if (mpack_tree_destroy(&tree) != mpack_ok) {
        fprintf(stderr, "error printing or destroying tree!\n");
        free(input_data);
        return EXIT_FAILURE;
    }

    free(input_data);
    return EXIT_SUCCESS;
}

#else
typedef int mpack_pedantic_allow_empty_translation_unit;
#endif
