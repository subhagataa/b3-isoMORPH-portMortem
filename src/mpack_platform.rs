#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn test_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn test_free(p: *mut ::core::ffi::c_void);
    fn test_strlen(s: *const ::core::ffi::c_char) -> size_t;
    fn mpack_assert_fail(message: *const ::core::ffi::c_char);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mpack_break_hit(message: *const ::core::ffi::c_char);
    fn mpack_tag_cmp(left: mpack_tag_t, right: mpack_tag_t) -> ::core::ffi::c_int;
    fn mpack_print_append(
        print: *mut mpack_print_t,
        data: *const ::core::ffi::c_char,
        count: size_t,
    );
    fn mpack_track_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_peek_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_bytes(track: *mut mpack_track_t, read: bool, count: size_t) -> mpack_error_t;
    fn mpack_track_str_bytes_all(
        track: *mut mpack_track_t,
        read: bool,
        count: size_t,
    ) -> mpack_error_t;
    fn mpack_writer_track_pop(writer: *mut mpack_writer_t, type_0: mpack_type_t);
    fn mpack_writer_init_filename(
        writer: *mut mpack_writer_t,
        filename: *const ::core::ffi::c_char,
    );
    fn mpack_write_i64(writer: *mut mpack_writer_t, value: int64_t);
    fn mpack_write_u64(writer: *mut mpack_writer_t, value: uint64_t);
    fn mpack_write_timestamp(writer: *mut mpack_writer_t, seconds: int64_t, nanoseconds: uint32_t);
    fn mpack_reader_init_filename(
        reader: *mut mpack_reader_t,
        filename: *const ::core::ffi::c_char,
    );
    fn mpack_reader_flag_error(reader: *mut mpack_reader_t, error: mpack_error_t);
    fn mpack_read_bytes_alloc_impl(
        reader: *mut mpack_reader_t,
        count: size_t,
        null_terminated: bool,
    ) -> *mut ::core::ffi::c_char;
    fn mpack_done_type(reader: *mut mpack_reader_t, type_0: mpack_type_t);
    fn mpack_print_data_to_file(data: *const ::core::ffi::c_char, len: size_t, file: *mut FILE);
    fn mpack_reader_ensure_straddle(reader: *mut mpack_reader_t, count: size_t) -> bool;
    fn mpack_read_native_straddle(
        reader: *mut mpack_reader_t,
        p: *mut ::core::ffi::c_char,
        count: size_t,
    );
    fn mpack_expect_u32(reader: *mut mpack_reader_t) -> uint32_t;
    fn mpack_expect_i32(reader: *mut mpack_reader_t) -> int32_t;
    fn mpack_expect_u8_range(
        reader: *mut mpack_reader_t,
        min_value: uint8_t,
        max_value: uint8_t,
    ) -> uint8_t;
    fn mpack_expect_u16_range(
        reader: *mut mpack_reader_t,
        min_value: uint16_t,
        max_value: uint16_t,
    ) -> uint16_t;
    fn mpack_expect_u32_range(
        reader: *mut mpack_reader_t,
        min_value: uint32_t,
        max_value: uint32_t,
    ) -> uint32_t;
    fn mpack_expect_u64_range(
        reader: *mut mpack_reader_t,
        min_value: uint64_t,
        max_value: uint64_t,
    ) -> uint64_t;
    fn mpack_expect_i8_range(
        reader: *mut mpack_reader_t,
        min_value: int8_t,
        max_value: int8_t,
    ) -> int8_t;
    fn mpack_expect_i16_range(
        reader: *mut mpack_reader_t,
        min_value: int16_t,
        max_value: int16_t,
    ) -> int16_t;
    fn mpack_expect_i32_range(
        reader: *mut mpack_reader_t,
        min_value: int32_t,
        max_value: int32_t,
    ) -> int32_t;
    fn mpack_expect_i64_range(
        reader: *mut mpack_reader_t,
        min_value: int64_t,
        max_value: int64_t,
    ) -> int64_t;
    fn mpack_expect_map_range(
        reader: *mut mpack_reader_t,
        min_count: uint32_t,
        max_count: uint32_t,
    ) -> uint32_t;
    fn mpack_expect_array_range(
        reader: *mut mpack_reader_t,
        min_count: uint32_t,
        max_count: uint32_t,
    ) -> uint32_t;
    fn mpack_expect_str(reader: *mut mpack_reader_t) -> uint32_t;
    fn mpack_expect_str_match(
        reader: *mut mpack_reader_t,
        str: *const ::core::ffi::c_char,
        length: size_t,
    );
    fn mpack_expect_bin(reader: *mut mpack_reader_t) -> uint32_t;
    fn mpack_expect_ext(reader: *mut mpack_reader_t, type_0: *mut int8_t) -> uint32_t;
    fn mpack_tree_init_data(
        tree: *mut mpack_tree_t,
        data: *const ::core::ffi::c_char,
        length: size_t,
    );
    fn mpack_tree_init_filename(
        tree: *mut mpack_tree_t,
        filename: *const ::core::ffi::c_char,
        max_bytes: size_t,
    );
    fn mpack_node_print_to_file(node: mpack_node_t, file: *mut FILE);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
pub type mpack_version_t = ::core::ffi::c_uint;
pub const mpack_version_current: mpack_version_t = 5;
pub const mpack_version_v5: mpack_version_t = 5;
pub const mpack_version_v4: mpack_version_t = 4;
pub type mpack_error_t = ::core::ffi::c_uint;
pub const mpack_error_eof: mpack_error_t = 10;
pub const mpack_error_data: mpack_error_t = 9;
pub const mpack_error_bug: mpack_error_t = 8;
pub const mpack_error_memory: mpack_error_t = 7;
pub const mpack_error_too_big: mpack_error_t = 6;
pub const mpack_error_type: mpack_error_t = 5;
pub const mpack_error_unsupported: mpack_error_t = 4;
pub const mpack_error_invalid: mpack_error_t = 3;
pub const mpack_error_io: mpack_error_t = 2;
pub const mpack_ok: mpack_error_t = 0;
pub type mpack_type_t = ::core::ffi::c_uint;
pub const mpack_type_ext: mpack_type_t = 11;
pub const mpack_type_map: mpack_type_t = 10;
pub const mpack_type_array: mpack_type_t = 9;
pub const mpack_type_bin: mpack_type_t = 8;
pub const mpack_type_str: mpack_type_t = 7;
pub const mpack_type_double: mpack_type_t = 6;
pub const mpack_type_float: mpack_type_t = 5;
pub const mpack_type_uint: mpack_type_t = 4;
pub const mpack_type_int: mpack_type_t = 3;
pub const mpack_type_bool: mpack_type_t = 2;
pub const mpack_type_nil: mpack_type_t = 1;
pub const mpack_type_missing: mpack_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_timestamp_t {
    pub seconds: int64_t,
    pub nanoseconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_tag_t {
    pub type_0: mpack_type_t,
    pub exttype: int8_t,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: uint64_t,
    pub i: int64_t,
    pub b: bool,
    pub f: ::core::ffi::c_float,
    pub d: ::core::ffi::c_double,
    pub l: uint32_t,
    pub n: uint32_t,
}
pub type mpack_print_callback_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, size_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_print_t {
    pub buffer: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub count: size_t,
    pub callback: mpack_print_callback_t,
    pub context: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub f: ::core::ffi::c_float,
    pub u: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub d: ::core::ffi::c_double,
    pub u: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub f: ::core::ffi::c_float,
    pub u: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub d: ::core::ffi::c_double,
    pub u: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_track_element_t {
    pub type_0: mpack_type_t,
    pub left: uint32_t,
    pub key_needs_value: bool,
    pub builder: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_track_t {
    pub count: size_t,
    pub capacity: size_t,
    pub elements: *mut mpack_track_element_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_writer_t {
    pub version: mpack_version_t,
    pub flush: mpack_writer_flush_t,
    pub error_fn: mpack_writer_error_t,
    pub teardown: mpack_writer_teardown_t,
    pub context: *mut ::core::ffi::c_void,
    pub buffer: *mut ::core::ffi::c_char,
    pub position: *mut ::core::ffi::c_char,
    pub end: *mut ::core::ffi::c_char,
    pub error: mpack_error_t,
    pub track: mpack_track_t,
    pub reserved: [*mut ::core::ffi::c_void; 2],
    pub builder: mpack_builder_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_builder_t {
    pub current_build: *mut mpack_build_t,
    pub latest_build: *mut mpack_build_t,
    pub current_page: *mut mpack_builder_page_t,
    pub pages: *mut mpack_builder_page_t,
    pub stash_buffer: *mut ::core::ffi::c_char,
    pub stash_position: *mut ::core::ffi::c_char,
    pub stash_end: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_builder_page_t {
    pub next: *mut mpack_builder_page_t,
    pub bytes_used: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_build_t {
    pub parent: *mut mpack_build_t,
    pub bytes: size_t,
    pub count: uint32_t,
    pub type_0: mpack_type_t,
    pub nested_compound_elements: uint32_t,
    pub key_needs_value: bool,
}
pub type mpack_writer_teardown_t = Option<unsafe extern "C" fn(*mut mpack_writer_t) -> ()>;
pub type mpack_writer_error_t =
    Option<unsafe extern "C" fn(*mut mpack_writer_t, mpack_error_t) -> ()>;
pub type mpack_writer_flush_t =
    Option<unsafe extern "C" fn(*mut mpack_writer_t, *const ::core::ffi::c_char, size_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_reader_t {
    pub context: *mut ::core::ffi::c_void,
    pub fill: mpack_reader_fill_t,
    pub error_fn: mpack_reader_error_t,
    pub teardown: mpack_reader_teardown_t,
    pub skip: mpack_reader_skip_t,
    pub buffer: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub data: *const ::core::ffi::c_char,
    pub end: *const ::core::ffi::c_char,
    pub error: mpack_error_t,
    pub track: mpack_track_t,
}
pub type mpack_reader_skip_t = Option<unsafe extern "C" fn(*mut mpack_reader_t, size_t) -> ()>;
pub type mpack_reader_teardown_t = Option<unsafe extern "C" fn(*mut mpack_reader_t) -> ()>;
pub type mpack_reader_error_t =
    Option<unsafe extern "C" fn(*mut mpack_reader_t, mpack_error_t) -> ()>;
pub type mpack_reader_fill_t =
    Option<unsafe extern "C" fn(*mut mpack_reader_t, *mut ::core::ffi::c_char, size_t) -> size_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_node_t {
    pub data: *mut mpack_node_data_t,
    pub tree: *mut mpack_tree_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_tree_t {
    pub error_fn: mpack_tree_error_t,
    pub read_fn: mpack_tree_read_t,
    pub teardown: mpack_tree_teardown_t,
    pub context: *mut ::core::ffi::c_void,
    pub nil_node: mpack_node_data_t,
    pub missing_node: mpack_node_data_t,
    pub error: mpack_error_t,
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_capacity: size_t,
    pub data: *const ::core::ffi::c_char,
    pub data_length: size_t,
    pub size: size_t,
    pub node_count: size_t,
    pub max_size: size_t,
    pub max_nodes: size_t,
    pub parser: mpack_tree_parser_t,
    pub root: *mut mpack_node_data_t,
    pub pool: *mut mpack_node_data_t,
    pub pool_count: size_t,
    pub next: *mut mpack_tree_page_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_tree_page_t {
    pub next: *mut mpack_tree_page_t,
    pub nodes: [mpack_node_data_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_node_data_t {
    pub type_0: mpack_type_t,
    pub len: uint32_t,
    pub value: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub b: bool,
    pub f: ::core::ffi::c_float,
    pub d: ::core::ffi::c_double,
    pub i: int64_t,
    pub u: uint64_t,
    pub offset: size_t,
    pub children: *mut mpack_node_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_tree_parser_t {
    pub state: mpack_tree_parse_state_t,
    pub possible_nodes_left: size_t,
    pub nodes: *mut mpack_node_data_t,
    pub nodes_left: size_t,
    pub current_node_reserved: size_t,
    pub level: size_t,
    pub stack: *mut mpack_level_t,
    pub stack_capacity: size_t,
    pub stack_owned: bool,
    pub stack_local: [mpack_level_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_level_t {
    pub child: *mut mpack_node_data_t,
    pub left: size_t,
}
pub type mpack_tree_parse_state_t = ::core::ffi::c_uint;
pub const mpack_tree_parse_state_parsed: mpack_tree_parse_state_t = 2;
pub const mpack_tree_parse_state_in_progress: mpack_tree_parse_state_t = 1;
pub const mpack_tree_parse_state_not_started: mpack_tree_parse_state_t = 0;
pub type mpack_tree_teardown_t = Option<unsafe extern "C" fn(*mut mpack_tree_t) -> ()>;
pub type mpack_tree_read_t =
    Option<unsafe extern "C" fn(*mut mpack_tree_t, *mut ::core::ffi::c_char, size_t) -> size_t>;
pub type mpack_tree_error_t = Option<unsafe extern "C" fn(*mut mpack_tree_t, mpack_error_t) -> ()>;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MPACK_INT_MIN: ::core::ffi::c_int = INT_MIN;
pub const MPACK_INT_MAX: ::core::ffi::c_int = INT_MAX;
pub const MPACK_UINT_MAX: ::core::ffi::c_uint = UINT_MAX;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_nil() -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_nil;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_bool(mut value: bool) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_bool;
    ret.v.b = value;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_true() -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_bool;
    ret.v.b = true_0 != 0;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_false() -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_bool;
    ret.v.b = false_0 != 0;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_int(mut value: int64_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_int;
    ret.v.i = value;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_uint(mut value: uint64_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_uint;
    ret.v.u = value;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_float(mut value: ::core::ffi::c_float) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_float;
    ret.v.f = value;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_double(mut value: ::core::ffi::c_double) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_double;
    ret.v.d = value;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_array(mut count: uint32_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_array;
    ret.v.n = count;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_map(mut count: uint32_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_map;
    ret.v.n = count;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_str(mut length: uint32_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_str;
    ret.v.l = length;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_bin(mut length: uint32_t) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_bin;
    ret.v.l = length;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_make_ext(
    mut exttype: int8_t,
    mut length: uint32_t,
) -> mpack_tag_t {
    let mut ret: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    ret.type_0 = mpack_type_ext;
    ret.exttype = exttype;
    ret.v.l = length;
    return ret;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_type(mut tag: *mut mpack_tag_t) -> mpack_type_t {
    return (*tag).type_0;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_bool_value(mut tag: *mut mpack_tag_t) -> bool {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_bool as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:444\n%s\ntag is not a bool!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_bool\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.b;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_int_value(mut tag: *mut mpack_tag_t) -> int64_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:462\n%s\ntag is not an int!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_int\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.i;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_uint_value(mut tag: *mut mpack_tag_t) -> uint64_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:480\n%s\ntag is not a uint!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_uint\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.u;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_float_value(mut tag: *mut mpack_tag_t) -> ::core::ffi::c_float {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:502\n%s\ntag is not a float!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_float\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.f;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_double_value(
    mut tag: *mut mpack_tag_t,
) -> ::core::ffi::c_double {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:524\n%s\ntag is not a double!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_double\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.d;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_array_count(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:537\n%s\ntag is not an array!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.n;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_map_count(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:550\n%s\ntag is not a map!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_map\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.n;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_str_length(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:563\n%s\ntag is not a str!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_str\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.l;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_bin_length(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:576\n%s\ntag is not a bin!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_bin\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.l;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_ext_length(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:592\n%s\ntag is not an ext!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_ext\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.l;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_ext_exttype(mut tag: *mut mpack_tag_t) -> int8_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:607\n%s\ntag is not an ext!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_ext\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).exttype;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_bytes(mut tag: *mut mpack_tag_t) -> uint32_t {
    if !((*tag).type_0 as ::core::ffi::c_uint
        == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*tag).type_0 as ::core::ffi::c_uint
            == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*tag).type_0 as ::core::ffi::c_uint
            == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.h:626\n%s\ntag is not a str, bin or ext!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"tag->type == mpack_type_str || tag->type == mpack_type_bin || tag->type == mpack_type_ext\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*tag).v.l;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_equal(mut left: mpack_tag_t, mut right: mpack_tag_t) -> bool {
    return mpack_tag_cmp(left, right) == 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_print_append_cstr(
    mut print: *mut mpack_print_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    mpack_print_append(print, cstr, test_strlen(cstr));
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_nil() -> mpack_tag_t {
    return mpack_tag_make_nil();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_bool(mut value: bool) -> mpack_tag_t {
    return mpack_tag_make_bool(value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_true() -> mpack_tag_t {
    return mpack_tag_make_true();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_false() -> mpack_tag_t {
    return mpack_tag_make_false();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_int(mut value: int64_t) -> mpack_tag_t {
    return mpack_tag_make_int(value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_uint(mut value: uint64_t) -> mpack_tag_t {
    return mpack_tag_make_uint(value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_float(mut value: ::core::ffi::c_float) -> mpack_tag_t {
    return mpack_tag_make_float(value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_double(mut value: ::core::ffi::c_double) -> mpack_tag_t {
    return mpack_tag_make_double(value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_array(mut count: int32_t) -> mpack_tag_t {
    return mpack_tag_make_array(count as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_map(mut count: int32_t) -> mpack_tag_t {
    return mpack_tag_make_map(count as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_str(mut length: int32_t) -> mpack_tag_t {
    return mpack_tag_make_str(length as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_bin(mut length: int32_t) -> mpack_tag_t {
    return mpack_tag_make_bin(length as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tag_ext(mut exttype: int8_t, mut length: int32_t) -> mpack_tag_t {
    return mpack_tag_make_ext(exttype, length as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_u8(mut p: *const ::core::ffi::c_char) -> uint8_t {
    return *p.offset(0 as ::core::ffi::c_int as isize) as uint8_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_u16(mut p: *const ::core::ffi::c_char) -> uint16_t {
    let mut val: uint16_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint16_t>() as size_t,
    );
    return val.swap_bytes();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_u32(mut p: *const ::core::ffi::c_char) -> uint32_t {
    let mut val: uint32_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return val.swap_bytes();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_u64(mut p: *const ::core::ffi::c_char) -> uint64_t {
    let mut val: uint64_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    return val.swap_bytes();
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_u8(mut p: *mut ::core::ffi::c_char, mut val: uint8_t) {
    let mut u: *mut uint8_t = p as *mut uint8_t;
    *u.offset(0 as ::core::ffi::c_int as isize) = val;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_u16(mut p: *mut ::core::ffi::c_char, mut val: uint16_t) {
    val = val.swap_bytes() as uint16_t;
    memcpy(
        p as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint16_t>() as size_t,
    );
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_u32(mut p: *mut ::core::ffi::c_char, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        p as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_u64(mut p: *mut ::core::ffi::c_char, mut val: uint64_t) {
    val = val.swap_bytes() as uint64_t;
    memcpy(
        p as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_i8(mut p: *const ::core::ffi::c_char) -> int8_t {
    return mpack_load_u8(p) as int8_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_i16(mut p: *const ::core::ffi::c_char) -> int16_t {
    return mpack_load_u16(p) as int16_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_i32(mut p: *const ::core::ffi::c_char) -> int32_t {
    return mpack_load_u32(p) as int32_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_i64(mut p: *const ::core::ffi::c_char) -> int64_t {
    return mpack_load_u64(p) as int64_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_i8(mut p: *mut ::core::ffi::c_char, mut val: int8_t) {
    mpack_store_u8(p, val as uint8_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_i16(mut p: *mut ::core::ffi::c_char, mut val: int16_t) {
    mpack_store_u16(p, val as uint16_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_i32(mut p: *mut ::core::ffi::c_char, mut val: int32_t) {
    mpack_store_u32(p, val as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_i64(mut p: *mut ::core::ffi::c_char, mut val: int64_t) {
    mpack_store_u64(p, val as uint64_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_float(
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_float {
    let mut v: C2RustUnnamed_0 = C2RustUnnamed_0 { f: 0. };
    v.u = mpack_load_u32(p);
    return v.f;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_load_double(
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    let mut v: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
    v.u = mpack_load_u64(p);
    return v.d;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_float(
    mut p: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_float,
) {
    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 { f: 0. };
    v.f = value;
    mpack_store_u32(p, v.u);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_store_double(
    mut p: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) {
    let mut v: C2RustUnnamed_3 = C2RustUnnamed_3 { d: 0. };
    v.d = value;
    mpack_store_u64(p, v.u);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_init_file(
    mut writer: *mut mpack_writer_t,
    mut filename: *const ::core::ffi::c_char,
) {
    mpack_writer_init_filename(writer, filename);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_set_version(
    mut writer: *mut mpack_writer_t,
    mut version: mpack_version_t,
) {
    (*writer).version = version;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_set_context(
    mut writer: *mut mpack_writer_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*writer).context = context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_context(
    mut writer: *mut mpack_writer_t,
) -> *mut ::core::ffi::c_void {
    return (*writer).context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_set_error_handler(
    mut writer: *mut mpack_writer_t,
    mut error_fn: mpack_writer_error_t,
) {
    (*writer).error_fn = error_fn;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_set_teardown(
    mut writer: *mut mpack_writer_t,
    mut teardown: mpack_writer_teardown_t,
) {
    (*writer).teardown = teardown;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_buffer_used(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).position.offset_from((*writer).buffer) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_buffer_left(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).end.offset_from((*writer).position) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_buffer_size(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).end.offset_from((*writer).buffer) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_writer_error(mut writer: *mut mpack_writer_t) -> mpack_error_t {
    return (*writer).error;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_write_int(mut writer: *mut mpack_writer_t, mut value: int64_t) {
    mpack_write_i64(writer, value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_write_uint(mut writer: *mut mpack_writer_t, mut value: uint64_t) {
    mpack_write_u64(writer, value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_write_timestamp_seconds(
    mut writer: *mut mpack_writer_t,
    mut seconds: int64_t,
) {
    mpack_write_timestamp(writer, seconds, 0 as uint32_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_write_timestamp_struct(
    mut writer: *mut mpack_writer_t,
    mut timestamp: mpack_timestamp_t,
) {
    mpack_write_timestamp(writer, timestamp.seconds, timestamp.nanoseconds);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_builder_compound_push(mut writer: *mut mpack_writer_t) {
    let mut build: *mut mpack_build_t = (*writer).builder.current_build;
    if !build.is_null() {
        (*build).nested_compound_elements = (*build).nested_compound_elements.wrapping_add(1);
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_builder_compound_pop(mut writer: *mut mpack_writer_t) {
    let mut build: *mut mpack_build_t = (*writer).builder.current_build;
    if !build.is_null() {
        if !((*build).nested_compound_elements > 0 as uint32_t) {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-writer.h:725\n%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"build->nested_compound_elements > 0\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        (*build).nested_compound_elements = (*build).nested_compound_elements.wrapping_sub(1);
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_array(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_array);
    mpack_builder_compound_pop(writer);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_map(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_map);
    mpack_builder_compound_pop(writer);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_str(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_str);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_bin(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_bin);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_ext(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_ext);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_finish_type(
    mut writer: *mut mpack_writer_t,
    mut type_0: mpack_type_t,
) {
    mpack_writer_track_pop(writer, type_0);
}
pub const MPACK_READER_SMALL_FRACTION_DENOMINATOR: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_init_file(
    mut reader: *mut mpack_reader_t,
    mut filename: *const ::core::ffi::c_char,
) {
    mpack_reader_init_filename(reader, filename);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_set_context(
    mut reader: *mut mpack_reader_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*reader).context = context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_context(
    mut reader: *mut mpack_reader_t,
) -> *mut ::core::ffi::c_void {
    return (*reader).context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_set_error_handler(
    mut reader: *mut mpack_reader_t,
    mut error_fn: mpack_reader_error_t,
) {
    (*reader).error_fn = error_fn;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_set_teardown(
    mut reader: *mut mpack_reader_t,
    mut teardown: mpack_reader_teardown_t,
) {
    (*reader).teardown = teardown;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_error(mut reader: *mut mpack_reader_t) -> mpack_error_t {
    return (*reader).error;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_flag_if_error(
    mut reader: *mut mpack_reader_t,
    mut error: mpack_error_t,
) -> mpack_error_t {
    if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        mpack_reader_flag_error(reader, error);
    }
    return mpack_reader_error(reader);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_read_bytes_alloc(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> *mut ::core::ffi::c_char {
    return mpack_read_bytes_alloc_impl(reader, count, false_0 != 0);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_should_read_bytes_inplace(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> bool {
    return (*reader).size == 0 as size_t
        || count
            <= (*reader)
                .size
                .wrapping_div(MPACK_READER_SMALL_FRACTION_DENOMINATOR as size_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_done_array(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_array);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_done_map(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_map);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_done_str(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_str);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_done_bin(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_bin);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_done_ext(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_ext);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_print_data_to_stdout(
    mut data: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    mpack_print_data_to_file(data, len, stdout);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_print(mut data: *const ::core::ffi::c_char, mut len: size_t) {
    mpack_print_data_to_stdout(data, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_ensure(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> bool {
    if !(count != 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.h:906\n%s\ncannot ensure zero bytes!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.h:907\n%s\nreader cannot be in an error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if count <= (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t {
        return true_0 != 0;
    }
    return mpack_reader_ensure_straddle(reader, count);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_read_native(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !p.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.h:919\n%s\ndata pointer for %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || p != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if count > (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t {
        mpack_read_native_straddle(reader, p, count);
    } else {
        memcpy(
            p as *mut ::core::ffi::c_void,
            (*reader).data as *const ::core::ffi::c_void,
            count,
        );
        (*reader).data = (*reader).data.offset(count as isize);
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_track_element(
    mut reader: *mut mpack_reader_t,
) -> mpack_error_t {
    return (if (*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_if_error(
            reader,
            mpack_track_element(&raw mut (*reader).track, 1 as ::core::ffi::c_int != 0),
        ) as ::core::ffi::c_uint
    } else {
        (*reader).error as ::core::ffi::c_uint
    }) as mpack_error_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_track_peek_element(
    mut reader: *mut mpack_reader_t,
) -> mpack_error_t {
    return (if (*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_if_error(
            reader,
            mpack_track_peek_element(&raw mut (*reader).track, 1 as ::core::ffi::c_int != 0),
        ) as ::core::ffi::c_uint
    } else {
        (*reader).error as ::core::ffi::c_uint
    }) as mpack_error_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_track_bytes(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> mpack_error_t {
    return (if (*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_if_error(
            reader,
            mpack_track_bytes(
                &raw mut (*reader).track,
                1 as ::core::ffi::c_int != 0,
                count,
            ),
        ) as ::core::ffi::c_uint
    } else {
        (*reader).error as ::core::ffi::c_uint
    }) as mpack_error_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_reader_track_str_bytes_all(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> mpack_error_t {
    return (if (*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_if_error(
            reader,
            mpack_track_str_bytes_all(
                &raw mut (*reader).track,
                1 as ::core::ffi::c_int != 0,
                count,
            ),
        ) as ::core::ffi::c_uint
    } else {
        (*reader).error as ::core::ffi::c_uint
    }) as mpack_error_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_uint_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: ::core::ffi::c_uint,
    mut max_value: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if ::core::mem::size_of::<::core::ffi::c_uint>() as usize == 4 as usize {
        return mpack_expect_u32_range(reader, min_value as uint32_t, max_value as uint32_t)
            as ::core::ffi::c_uint;
    }
    return mpack_expect_u64_range(reader, min_value as uint64_t, max_value as uint64_t)
        as ::core::ffi::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_u8_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: uint8_t,
) -> uint8_t {
    return mpack_expect_u8_range(reader, 0 as uint8_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_u16_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: uint16_t,
) -> uint16_t {
    return mpack_expect_u16_range(reader, 0 as uint16_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_u32_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: uint32_t,
) -> uint32_t {
    return mpack_expect_u32_range(reader, 0 as uint32_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_u64_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: uint64_t,
) -> uint64_t {
    return mpack_expect_u64_range(reader, 0 as uint64_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_uint_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return mpack_expect_uint_range(reader, 0 as ::core::ffi::c_uint, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_int_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: ::core::ffi::c_int,
    mut max_value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ::core::mem::size_of::<::core::ffi::c_int>() as usize == 4 as usize {
        return mpack_expect_i32_range(reader, min_value as int32_t, max_value as int32_t)
            as ::core::ffi::c_int;
    }
    return mpack_expect_i64_range(reader, min_value as int64_t, max_value as int64_t)
        as ::core::ffi::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_i8_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: int8_t,
) -> int8_t {
    return mpack_expect_i8_range(reader, 0 as int8_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_i16_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: int16_t,
) -> int16_t {
    return mpack_expect_i16_range(reader, 0 as int16_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_i32_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: int32_t,
) -> int32_t {
    return mpack_expect_i32_range(reader, 0 as int32_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_i64_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: int64_t,
) -> int64_t {
    return mpack_expect_i64_range(reader, 0 as int64_t, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_int_max(
    mut reader: *mut mpack_reader_t,
    mut max_value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return mpack_expect_int_range(reader, 0 as ::core::ffi::c_int, max_value);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_uint(mut reader: *mut mpack_reader_t) -> ::core::ffi::c_uint {
    if ::core::mem::size_of::<::core::ffi::c_uint>() as usize == 4 as usize {
        return mpack_expect_u32(reader) as ::core::ffi::c_uint;
    }
    return mpack_expect_u64_max(reader, MPACK_UINT_MAX as uint64_t) as ::core::ffi::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_int(mut reader: *mut mpack_reader_t) -> ::core::ffi::c_int {
    if ::core::mem::size_of::<::core::ffi::c_int>() as usize == 4 as usize {
        return mpack_expect_i32(reader) as ::core::ffi::c_int;
    }
    return mpack_expect_i64_range(reader, MPACK_INT_MIN as int64_t, MPACK_INT_MAX as int64_t)
        as ::core::ffi::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_map_max(
    mut reader: *mut mpack_reader_t,
    mut max_count: uint32_t,
) -> uint32_t {
    return mpack_expect_map_range(reader, 0 as uint32_t, max_count);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_array_max(
    mut reader: *mut mpack_reader_t,
    mut max_count: uint32_t,
) -> uint32_t {
    return mpack_expect_array_range(reader, 0 as uint32_t, max_count);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_str_max(
    mut reader: *mut mpack_reader_t,
    mut maxsize: uint32_t,
) -> uint32_t {
    let mut length: uint32_t = mpack_expect_str(reader);
    if length > maxsize {
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return 0 as uint32_t;
    }
    return length;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_str_length(
    mut reader: *mut mpack_reader_t,
    mut count: uint32_t,
) {
    if mpack_expect_str(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_cstr_match(
    mut reader: *mut mpack_reader_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.h:1080\n%s\ncstr pointer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_expect_str_match(reader, cstr, test_strlen(cstr));
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_bin_max(
    mut reader: *mut mpack_reader_t,
    mut maxsize: uint32_t,
) -> uint32_t {
    let mut length: uint32_t = mpack_expect_bin(reader);
    if length > maxsize {
        mpack_reader_flag_error(reader, mpack_error_type);
        return 0 as uint32_t;
    }
    return length;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_bin_size(
    mut reader: *mut mpack_reader_t,
    mut count: uint32_t,
) {
    if mpack_expect_bin(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_ext_max(
    mut reader: *mut mpack_reader_t,
    mut type_0: *mut int8_t,
    mut maxsize: uint32_t,
) -> uint32_t {
    let mut length: uint32_t = mpack_expect_ext(reader, type_0);
    if length > maxsize {
        mpack_reader_flag_error(reader, mpack_error_type);
        return 0 as uint32_t;
    }
    return length;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_expect_ext_size(
    mut reader: *mut mpack_reader_t,
    mut type_0: *mut int8_t,
    mut count: uint32_t,
) {
    if mpack_expect_ext(reader, type_0) != count {
        *type_0 = 0 as int8_t;
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_node(
    mut tree: *mut mpack_tree_t,
    mut data: *mut mpack_node_data_t,
) -> mpack_node_t {
    let mut node: mpack_node_t = mpack_node_t {
        data: ::core::ptr::null_mut::<mpack_node_data_t>(),
        tree: ::core::ptr::null_mut::<mpack_tree_t>(),
    };
    node.data = data;
    node.tree = tree;
    return node;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_node_child(
    mut node: mpack_node_t,
    mut child: size_t,
) -> *mut mpack_node_data_t {
    return (*node.data).value.children.offset(child as isize);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_nil_node(mut tree: *mut mpack_tree_t) -> mpack_node_t {
    return mpack_node(tree, &raw mut (*tree).nil_node);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_missing_node(mut tree: *mut mpack_tree_t) -> mpack_node_t {
    return mpack_node(tree, &raw mut (*tree).missing_node);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_init(
    mut tree: *mut mpack_tree_t,
    mut data: *const ::core::ffi::c_char,
    mut length: size_t,
) {
    mpack_tree_init_data(tree, data, length);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_init_file(
    mut tree: *mut mpack_tree_t,
    mut filename: *const ::core::ffi::c_char,
    mut max_bytes: size_t,
) {
    mpack_tree_init_filename(tree, filename, max_bytes);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_error(mut tree: *mut mpack_tree_t) -> mpack_error_t {
    return (*tree).error;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_size(mut tree: *mut mpack_tree_t) -> size_t {
    return (*tree).size;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_set_context(
    mut tree: *mut mpack_tree_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*tree).context = context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_context(
    mut tree: *mut mpack_tree_t,
) -> *mut ::core::ffi::c_void {
    return (*tree).context;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_set_error_handler(
    mut tree: *mut mpack_tree_t,
    mut error_fn: mpack_tree_error_t,
) {
    (*tree).error_fn = error_fn;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_tree_set_teardown(
    mut tree: *mut mpack_tree_t,
    mut teardown: mpack_tree_teardown_t,
) {
    (*tree).teardown = teardown;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_node_error(mut node: mpack_node_t) -> mpack_error_t {
    return mpack_tree_error(node.tree);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_node_print_to_stdout(mut node: mpack_node_t) {
    mpack_node_print_to_file(node, stdout);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mpack_node_print(mut node: mpack_node_t) {
    mpack_node_print_to_stdout(node);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_assert_fail_format(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) -> ! {
    let mut buffer: [::core::ffi::c_char; 512] = [0; 512];
    let mut args_0;
    args_0 = args.clone();
    vsnprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        format,
        args_0,
    );
    buffer[(::core::mem::size_of::<[::core::ffi::c_char; 512]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    mpack_assert_fail_wrapper(&raw mut buffer as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_break_hit_format(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut buffer: [::core::ffi::c_char; 512] = [0; 512];
    let mut args_0;
    args_0 = args.clone();
    vsnprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        format,
        args_0,
    );
    buffer[(::core::mem::size_of::<[::core::ffi::c_char; 512]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    mpack_break_hit(&raw mut buffer as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_realloc(
    mut old_ptr: *mut ::core::ffi::c_void,
    mut used_size: size_t,
    mut new_size: size_t,
) -> *mut ::core::ffi::c_void {
    if new_size == 0 as size_t {
        if !old_ptr.is_null() {
            test_free(old_ptr);
        }
        return NULL;
    }
    let mut new_ptr: *mut ::core::ffi::c_void = test_malloc(new_size);
    if new_ptr.is_null() {
        return NULL;
    }
    memcpy(new_ptr, old_ptr, used_size);
    test_free(old_ptr);
    return new_ptr;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;

#[no_mangle]
pub unsafe extern "C" fn mpack_assert_fail_wrapper(message: *mut ::core::ffi::c_char) -> ! {
    mpack_assert_fail(message as *const ::core::ffi::c_char);
    // mpack_assert_fail is not supposed to return; abort as a fallback,
    // matching the original C implementation's behavior.
    ::std::process::abort()
}
