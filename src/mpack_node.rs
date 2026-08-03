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
    fn test_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn test_free(p: *mut ::core::ffi::c_void);
    fn test_strlen(s: *const ::core::ffi::c_char) -> size_t;
    fn test_fopen(path: *const ::core::ffi::c_char, mode: *const ::core::ffi::c_char) -> *mut FILE;
    fn test_fclose(stream: *mut FILE) -> ::core::ffi::c_int;
    fn test_fread(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
    fn test_fseek(
        stream: *mut FILE,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn test_ftell(stream: *mut FILE) -> ::core::ffi::c_long;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn mpack_assert_fail_format(format: *const ::core::ffi::c_char, ...) -> !;
    fn mpack_break_hit_format(format: *const ::core::ffi::c_char, ...);
    fn mpack_realloc(
        old_ptr: *mut ::core::ffi::c_void,
        used_size: size_t,
        new_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mpack_type_to_string(type_0: mpack_type_t) -> *const ::core::ffi::c_char;
    fn mpack_tag_debug_pseudo_json(
        tag: mpack_tag_t,
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
        prefix: *const ::core::ffi::c_char,
        prefix_size: size_t,
    );
    fn mpack_print_append(
        print: *mut mpack_print_t,
        data: *const ::core::ffi::c_char,
        count: size_t,
    );
    fn mpack_print_flush(print: *mut mpack_print_t);
    fn mpack_print_file_callback(
        context: *mut ::core::ffi::c_void,
        data: *const ::core::ffi::c_char,
        count: size_t,
    );
    fn mpack_utf8_check(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
    fn mpack_utf8_check_no_null(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
    fn mpack_str_check_no_null(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
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
    pub value: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpack_file_tree_t {
    pub data: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub buffer: [::core::ffi::c_char; 33],
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT8_MIN: ::core::ffi::c_int = -(128 as ::core::ffi::c_int);
pub const INT16_MIN: ::core::ffi::c_int = -(32767 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT8_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const INT16_MAX: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MPACK_INT8_MIN: ::core::ffi::c_int = INT8_MIN;
pub const MPACK_INT16_MIN: ::core::ffi::c_int = INT16_MIN;
pub const MPACK_INT32_MIN: ::core::ffi::c_int = INT32_MIN;
pub const MPACK_INT_MIN: ::core::ffi::c_int = INT_MIN;
pub const MPACK_INT8_MAX: ::core::ffi::c_int = INT8_MAX;
pub const MPACK_INT16_MAX: ::core::ffi::c_int = INT16_MAX;
pub const MPACK_INT32_MAX: ::core::ffi::c_int = INT32_MAX;
pub const MPACK_INT64_MAX: ::core::ffi::c_long = INT64_MAX;
pub const MPACK_INT_MAX: ::core::ffi::c_int = INT_MAX;
pub const MPACK_UINT8_MAX: ::core::ffi::c_int = UINT8_MAX;
pub const MPACK_UINT16_MAX: ::core::ffi::c_int = UINT16_MAX;
pub const MPACK_UINT32_MAX: ::core::ffi::c_uint = UINT32_MAX;
pub const MPACK_UINT_MAX: ::core::ffi::c_uint = UINT_MAX;
pub const MPACK_TIMESTAMP_NANOSECONDS_MAX: ::core::ffi::c_int = 999999999 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mpack_tag_make_nil() -> mpack_tag_t {
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
pub const MPACK_EXTTYPE_TIMESTAMP: int8_t = -(1 as ::core::ffi::c_int) as int8_t;
#[inline]
unsafe extern "C" fn mpack_print_append_cstr(
    mut print: *mut mpack_print_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    mpack_print_append(print, cstr, test_strlen(cstr));
}
#[inline]
unsafe extern "C" fn mpack_tag_nil() -> mpack_tag_t {
    return mpack_tag_make_nil();
}
#[inline]
unsafe extern "C" fn mpack_load_u8(mut p: *const ::core::ffi::c_char) -> uint8_t {
    return *p.offset(0 as ::core::ffi::c_int as isize) as uint8_t;
}
#[inline]
unsafe extern "C" fn mpack_load_u16(mut p: *const ::core::ffi::c_char) -> uint16_t {
    let mut val: uint16_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint16_t>() as size_t,
    );
    return val.swap_bytes();
}
#[inline]
unsafe extern "C" fn mpack_load_u32(mut p: *const ::core::ffi::c_char) -> uint32_t {
    let mut val: uint32_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return val.swap_bytes();
}
#[inline]
unsafe extern "C" fn mpack_load_u64(mut p: *const ::core::ffi::c_char) -> uint64_t {
    let mut val: uint64_t = 0;
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    return val.swap_bytes();
}
#[inline]
unsafe extern "C" fn mpack_load_i8(mut p: *const ::core::ffi::c_char) -> int8_t {
    return mpack_load_u8(p) as int8_t;
}
#[inline]
unsafe extern "C" fn mpack_load_i16(mut p: *const ::core::ffi::c_char) -> int16_t {
    return mpack_load_u16(p) as int16_t;
}
#[inline]
unsafe extern "C" fn mpack_load_i32(mut p: *const ::core::ffi::c_char) -> int32_t {
    return mpack_load_u32(p) as int32_t;
}
#[inline]
unsafe extern "C" fn mpack_load_i64(mut p: *const ::core::ffi::c_char) -> int64_t {
    return mpack_load_u64(p) as int64_t;
}
#[inline]
unsafe extern "C" fn mpack_load_float(mut p: *const ::core::ffi::c_char) -> ::core::ffi::c_float {
    let mut v: C2RustUnnamed_0 = C2RustUnnamed_0 { f: 0. };
    v.u = mpack_load_u32(p);
    return v.f;
}
#[inline]
unsafe extern "C" fn mpack_load_double(mut p: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    let mut v: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
    v.u = mpack_load_u64(p);
    return v.d;
}
#[inline]
unsafe extern "C" fn mpack_node(
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
#[inline]
unsafe extern "C" fn mpack_node_child(
    mut node: mpack_node_t,
    mut child: size_t,
) -> *mut mpack_node_data_t {
    return (*node.data).value.children.offset(child as isize);
}
#[inline]
unsafe extern "C" fn mpack_tree_nil_node(mut tree: *mut mpack_tree_t) -> mpack_node_t {
    return mpack_node(tree, &raw mut (*tree).nil_node);
}
#[inline]
unsafe extern "C" fn mpack_tree_missing_node(mut tree: *mut mpack_tree_t) -> mpack_node_t {
    return mpack_node(tree, &raw mut (*tree).missing_node);
}
#[inline]
unsafe extern "C" fn mpack_tree_error(tree: *mut mpack_tree_t) -> mpack_error_t {
    // SAFETY: FFI contract requires `tree` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let tree_ref: &mpack_tree_t = unsafe { &*tree };
    return tree_ref.error;
}
#[inline]
unsafe extern "C" fn mpack_tree_set_context(
    mut tree: *mut mpack_tree_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*tree).context = context;
}
#[inline]
unsafe extern "C" fn mpack_tree_set_teardown(
    mut tree: *mut mpack_tree_t,
    mut teardown: mpack_tree_teardown_t,
) {
    (*tree).teardown = teardown;
}
#[inline]
unsafe extern "C" fn mpack_node_error(mut node: mpack_node_t) -> mpack_error_t {
    return mpack_tree_error(node.tree);
}
#[inline]
unsafe extern "C" fn mpack_node_data_unchecked(
    mut node: mpack_node_t,
) -> *const ::core::ffi::c_char {
    if !(mpack_node_error(node) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:31\n%s\ntree is in an error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"mpack_node_error(node) == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if !(type_0 as ::core::ffi::c_uint
        == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:37\n%s\nnode of type %i (%s) is not a data type!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"type == mpack_type_str || type == mpack_type_bin || type == mpack_type_ext\0"
                as *const u8 as *const ::core::ffi::c_char,
            type_0 as ::core::ffi::c_uint,
            mpack_type_to_string(type_0),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return (*node.tree).data.offset((*node.data).value.offset as isize);
}
#[inline]
unsafe extern "C" fn mpack_node_exttype_unchecked(mut node: mpack_node_t) -> int8_t {
    if !(mpack_node_error(node) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:48\n%s\ntree is in an error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"mpack_node_error(node) == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if !(type_0 as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:53\n%s\nnode of type %i (%s) is not an ext type!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"type == mpack_type_ext\0" as *const u8 as *const ::core::ffi::c_char,
            type_0 as ::core::ffi::c_uint,
            mpack_type_to_string(type_0),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return mpack_load_i8(
        mpack_node_data_unchecked(node).offset(-(1 as ::core::ffi::c_int as isize)),
    );
}
pub const MPACK_NODES_PER_PAGE: usize = (MPACK_NODE_PAGE_SIZE as usize)
    .wrapping_sub(::core::mem::size_of::<mpack_tree_page_t>() as usize)
    .wrapping_div(::core::mem::size_of::<mpack_node_data_t>() as usize)
    .wrapping_add(1 as usize);
pub const MPACK_PAGE_ALLOC_SIZE: usize = (::core::mem::size_of::<mpack_tree_page_t>() as usize)
    .wrapping_add(
        (::core::mem::size_of::<mpack_node_data_t>() as usize)
            .wrapping_mul(MPACK_NODES_PER_PAGE.wrapping_sub(1 as usize)),
    );
unsafe extern "C" fn mpack_tree_reserve_fill(mut tree: *mut mpack_tree_t) -> bool {
    if !((*tree).parser.state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:85\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->parser.state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut bytes: size_t = (*tree).parser.current_node_reserved;
    if !(bytes > (*tree).parser.possible_nodes_left) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:89\n%s\nthere are already enough bytes! call mpack_tree_ensure() instead.\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"bytes > tree->parser.possible_nodes_left\0" as *const u8
                as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*tree).data_length.wrapping_add(bytes) > (*tree).max_size {
        mpack_tree_flag_error(tree, mpack_error_too_big);
        return false_0 != 0;
    }
    if (*tree).read_fn.is_none() {
        mpack_tree_flag_error(tree, mpack_error_invalid);
        return false_0 != 0;
    }
    if (*tree).data_length.wrapping_add(bytes) > (*tree).buffer_capacity {
        let mut new_capacity: size_t = if (*tree).buffer_capacity == 0 as size_t {
            MPACK_BUFFER_SIZE as size_t
        } else {
            (*tree).buffer_capacity
        };
        while new_capacity < (*tree).data_length.wrapping_add(bytes) {
            new_capacity = new_capacity.wrapping_mul(2 as size_t);
        }
        if new_capacity > (*tree).max_size {
            new_capacity = (*tree).max_size;
        }
        let mut new_buffer: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        if (*tree).buffer.is_null() {
            new_buffer = test_malloc(new_capacity) as *mut ::core::ffi::c_char;
        } else {
            new_buffer = mpack_realloc(
                (*tree).buffer as *mut ::core::ffi::c_void,
                (*tree).data_length,
                new_capacity,
            ) as *mut ::core::ffi::c_char;
        }
        if new_buffer.is_null() {
            mpack_tree_flag_error(tree, mpack_error_memory);
            return false_0 != 0;
        }
        (*tree).data = new_buffer;
        (*tree).buffer = new_buffer;
        (*tree).buffer_capacity = new_capacity;
    }
    loop {
        let mut read: size_t = (*tree).read_fn.expect("non-null function pointer")(
            tree,
            (*tree).buffer.offset((*tree).data_length as isize),
            (*tree).buffer_capacity.wrapping_sub((*tree).data_length),
        );
        if mpack_tree_error(tree) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
        if read == -(1 as ::core::ffi::c_int) as size_t {
            mpack_tree_flag_error(tree, mpack_error_io);
            return false_0 != 0;
        }
        if read == 0 as size_t {
            return false_0 != 0;
        }
        (*tree).data_length = (*tree).data_length.wrapping_add(read);
        (*tree).parser.possible_nodes_left = (*tree).parser.possible_nodes_left.wrapping_add(read);
        if !((*tree).parser.possible_nodes_left < bytes) {
            break;
        }
    }
    return true_0 != 0;
}
#[inline]
unsafe extern "C" fn mpack_tree_reserve_bytes(
    mut tree: *mut mpack_tree_t,
    mut extra_bytes: size_t,
) -> bool {
    if !((*tree).parser.state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:184\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->parser.state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if ((*tree).parser.current_node_reserved as uint64_t).wrapping_add(extra_bytes as uint64_t)
        > SIZE_MAX as uint64_t
    {
        mpack_tree_flag_error(tree, mpack_error_invalid);
        return false_0 != 0;
    }
    (*tree).parser.current_node_reserved = (*tree)
        .parser
        .current_node_reserved
        .wrapping_add(extra_bytes);
    if (*tree).parser.current_node_reserved <= (*tree).parser.possible_nodes_left {
        return true_0 != 0;
    }
    return mpack_tree_reserve_fill(tree);
}
#[inline]
unsafe extern "C" fn mpack_tree_parser_stack_capacity(mut tree: *mut mpack_tree_t) -> size_t {
    return (*tree).parser.stack_capacity;
}
unsafe extern "C" fn mpack_tree_push_stack(
    mut tree: *mut mpack_tree_t,
    mut first_child: *mut mpack_node_data_t,
    mut total: size_t,
) -> bool {
    let mut parser: *mut mpack_tree_parser_t = &raw mut (*tree).parser;
    if !((*parser).state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:222\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"parser->state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if total == 0 as size_t {
        return true_0 != 0;
    }
    if (*parser).level.wrapping_add(1 as size_t) == mpack_tree_parser_stack_capacity(tree) {
        let mut new_capacity: size_t = (*parser).stack_capacity.wrapping_mul(2 as size_t);
        if !(*parser).stack_owned {
            let mut new_stack: *mut mpack_level_t = test_malloc(
                (::core::mem::size_of::<mpack_level_t>() as size_t).wrapping_mul(new_capacity),
            ) as *mut mpack_level_t;
            if new_stack.is_null() {
                mpack_tree_flag_error(tree, mpack_error_memory);
                return false_0 != 0;
            }
            memcpy(
                new_stack as *mut ::core::ffi::c_void,
                (*parser).stack as *const ::core::ffi::c_void,
                (::core::mem::size_of::<mpack_level_t>() as size_t)
                    .wrapping_mul((*parser).stack_capacity),
            );
            (*parser).stack = new_stack;
            (*parser).stack_owned = true_0 != 0;
        } else {
            let mut new_stack_0: *mut mpack_level_t = mpack_realloc(
                (*parser).stack as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<mpack_level_t>() as size_t)
                    .wrapping_mul((*parser).stack_capacity),
                (::core::mem::size_of::<mpack_level_t>() as size_t).wrapping_mul(new_capacity),
            ) as *mut mpack_level_t;
            if new_stack_0.is_null() {
                mpack_tree_flag_error(tree, mpack_error_memory);
                return false_0 != 0;
            }
            (*parser).stack = new_stack_0;
        }
        (*parser).stack_capacity = new_capacity;
    }
    (*parser).level = (*parser).level.wrapping_add(1);
    let ref mut fresh2 = (*(*parser).stack.offset((*parser).level as isize)).child;
    *fresh2 = first_child;
    (*(*parser).stack.offset((*parser).level as isize)).left = total;
    return true_0 != 0;
}
unsafe extern "C" fn mpack_tree_parse_children(
    mut tree: *mut mpack_tree_t,
    mut node: *mut mpack_node_data_t,
) -> bool {
    let mut parser: *mut mpack_tree_parser_t = &raw mut (*tree).parser;
    if !((*parser).state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:271\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"parser->state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: mpack_type_t = (*node).type_0;
    let mut total: size_t = (*node).len as size_t;
    if type_0 as ::core::ffi::c_uint == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (total as uint64_t).wrapping_mul(2 as uint64_t) > SIZE_MAX as uint64_t {
            mpack_tree_flag_error(tree, mpack_error_too_big);
            return false_0 != 0;
        }
        total = total.wrapping_mul(2 as size_t);
    }
    (*tree).node_count = (*tree).node_count.wrapping_add(total);
    if (*tree).node_count > (*tree).max_nodes {
        mpack_tree_flag_error(tree, mpack_error_too_big);
        return false_0 != 0;
    }
    if !mpack_tree_reserve_bytes(tree, total) {
        return false_0 != 0;
    }
    if total <= (*parser).nodes_left {
        (*node).value.children = (*parser).nodes;
        (*parser).nodes = (*parser).nodes.offset(total as isize);
        (*parser).nodes_left = (*parser).nodes_left.wrapping_sub(total);
    } else {
        if (*tree).next.is_null() {
            mpack_tree_flag_error(tree, mpack_error_too_big);
            return false_0 != 0;
        }
        let mut page: *mut mpack_tree_page_t = ::core::ptr::null_mut::<mpack_tree_page_t>();
        if total > MPACK_NODES_PER_PAGE
            || (*parser).nodes_left > MPACK_NODES_PER_PAGE.wrapping_div(8 as usize)
        {
            page = test_malloc(
                (::core::mem::size_of::<mpack_tree_page_t>() as size_t).wrapping_add(
                    (::core::mem::size_of::<mpack_node_data_t>() as size_t)
                        .wrapping_mul(total.wrapping_sub(1 as size_t)),
                ),
            ) as *mut mpack_tree_page_t;
            if page.is_null() {
                mpack_tree_flag_error(tree, mpack_error_memory);
                return false_0 != 0;
            }
            (*node).value.children = &raw mut (*page).nodes as *mut mpack_node_data_t;
        } else {
            page = test_malloc(MPACK_PAGE_ALLOC_SIZE) as *mut mpack_tree_page_t;
            if page.is_null() {
                mpack_tree_flag_error(tree, mpack_error_memory);
                return false_0 != 0;
            }
            (*node).value.children = &raw mut (*page).nodes as *mut mpack_node_data_t;
            (*parser).nodes =
                (&raw mut (*page).nodes as *mut mpack_node_data_t).offset(total as isize);
            (*parser).nodes_left = MPACK_NODES_PER_PAGE.wrapping_sub(total as usize) as size_t;
        }
        (*page).next = (*tree).next as *mut mpack_tree_page_t;
        (*tree).next = page;
    }
    return mpack_tree_push_stack(tree, (*node).value.children, total);
}
unsafe extern "C" fn mpack_tree_parse_bytes(
    mut tree: *mut mpack_tree_t,
    mut node: *mut mpack_node_data_t,
) -> bool {
    (*node).value.offset = (*tree)
        .size
        .wrapping_add((*tree).parser.current_node_reserved)
        .wrapping_add(1 as size_t);
    return mpack_tree_reserve_bytes(tree, (*node).len as size_t);
}
unsafe extern "C" fn mpack_tree_parse_ext(
    mut tree: *mut mpack_tree_t,
    mut node: *mut mpack_node_data_t,
) -> bool {
    (*tree).parser.current_node_reserved = ((*tree).parser.current_node_reserved
        as ::core::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<int8_t>() as usize as ::core::ffi::c_ulong)
        as size_t as size_t;
    (*node).type_0 = mpack_type_ext;
    return mpack_tree_parse_bytes(tree, node);
}
unsafe extern "C" fn mpack_tree_parse_node_contents(
    mut tree: *mut mpack_tree_t,
    mut node: *mut mpack_node_data_t,
) -> bool {
    if !((*tree).parser.state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:379\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->parser.state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if node.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:380\n%s\nnull node?\0" as *const u8
                as *const ::core::ffi::c_char,
            b"node != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*tree).data_length > (*tree).size) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:385\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->data_length > tree->size\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: uint8_t = mpack_load_u8((*tree).data.offset((*tree).size as isize));
    (*tree).parser.current_node_reserved = 0 as size_t;
    match type_0 as ::core::ffi::c_int {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
        | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36
        | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53
        | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70
        | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87
        | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103
        | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117
        | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 => {
            (*node).type_0 = mpack_type_uint;
            (*node).value.u = type_0 as uint64_t;
            return true_0 != 0;
        }
        224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237 | 238
        | 239 | 240 | 241 | 242 | 243 | 244 | 245 | 246 | 247 | 248 | 249 | 250 | 251 | 252
        | 253 | 254 | 255 => {
            (*node).type_0 = mpack_type_int;
            (*node).value.i = type_0 as int8_t as int64_t;
            return true_0 != 0;
        }
        128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142
        | 143 => {
            (*node).type_0 = mpack_type_map;
            (*node).len =
                (type_0 as ::core::ffi::c_int & !(0xf0 as ::core::ffi::c_int)) as uint32_t;
            return mpack_tree_parse_children(tree, node);
        }
        144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158
        | 159 => {
            (*node).type_0 = mpack_type_array;
            (*node).len =
                (type_0 as ::core::ffi::c_int & !(0xf0 as ::core::ffi::c_int)) as uint32_t;
            return mpack_tree_parse_children(tree, node);
        }
        160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174
        | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188
        | 189 | 190 | 191 => {
            (*node).type_0 = mpack_type_str;
            (*node).len =
                (type_0 as ::core::ffi::c_int & !(0xe0 as ::core::ffi::c_int)) as uint32_t;
            return mpack_tree_parse_bytes(tree, node);
        }
        192 => {
            (*node).type_0 = mpack_type_nil;
            return true_0 != 0;
        }
        194 | 195 => {
            (*node).type_0 = mpack_type_bool;
            (*node).value.b = type_0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0;
            return true_0 != 0;
        }
        196 => {
            (*node).type_0 = mpack_type_bin;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint8_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u8(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            return mpack_tree_parse_bytes(tree, node);
        }
        197 => {
            (*node).type_0 = mpack_type_bin;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            return mpack_tree_parse_bytes(tree, node);
        }
        198 => {
            (*node).type_0 = mpack_type_bin;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            return mpack_tree_parse_bytes(tree, node);
        }
        199 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint8_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u8(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        200 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        201 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            return mpack_tree_parse_ext(tree, node);
        }
        202 => {
            if !mpack_tree_reserve_bytes(
                tree,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            ) {
                return false_0 != 0;
            }
            (*node).value.f = mpack_load_float(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            (*node).type_0 = mpack_type_float;
            return true_0 != 0;
        }
        203 => {
            if !mpack_tree_reserve_bytes(
                tree,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            ) {
                return false_0 != 0;
            }
            (*node).value.d = mpack_load_double(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            (*node).type_0 = mpack_type_double;
            return true_0 != 0;
        }
        204 => {
            (*node).type_0 = mpack_type_uint;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint8_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.u = mpack_load_u8(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t;
            return true_0 != 0;
        }
        205 => {
            (*node).type_0 = mpack_type_uint;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.u = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t;
            return true_0 != 0;
        }
        206 => {
            (*node).type_0 = mpack_type_uint;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.u = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t;
            return true_0 != 0;
        }
        207 => {
            (*node).type_0 = mpack_type_uint;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint64_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.u = mpack_load_u64(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            return true_0 != 0;
        }
        208 => {
            (*node).type_0 = mpack_type_int;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<int8_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.i = mpack_load_i8(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t;
            return true_0 != 0;
        }
        209 => {
            (*node).type_0 = mpack_type_int;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<int16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.i = mpack_load_i16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t;
            return true_0 != 0;
        }
        210 => {
            (*node).type_0 = mpack_type_int;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<int32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.i = mpack_load_i32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t;
            return true_0 != 0;
        }
        211 => {
            (*node).type_0 = mpack_type_int;
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<int64_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).value.i = mpack_load_i64(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            return true_0 != 0;
        }
        212 => {
            (*node).len = 1 as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        213 => {
            (*node).len = 2 as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        214 => {
            (*node).len = 4 as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        215 => {
            (*node).len = 8 as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        216 => {
            (*node).len = 16 as uint32_t;
            return mpack_tree_parse_ext(tree, node);
        }
        217 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint8_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u8(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            (*node).type_0 = mpack_type_str;
            return mpack_tree_parse_bytes(tree, node);
        }
        218 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            (*node).type_0 = mpack_type_str;
            return mpack_tree_parse_bytes(tree, node);
        }
        219 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            (*node).type_0 = mpack_type_str;
            return mpack_tree_parse_bytes(tree, node);
        }
        220 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            (*node).type_0 = mpack_type_array;
            return mpack_tree_parse_children(tree, node);
        }
        221 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            (*node).type_0 = mpack_type_array;
            return mpack_tree_parse_children(tree, node);
        }
        222 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint16_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u16(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t;
            (*node).type_0 = mpack_type_map;
            return mpack_tree_parse_children(tree, node);
        }
        223 => {
            if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint32_t>() as size_t) {
                return false_0 != 0;
            }
            (*node).len = mpack_load_u32(
                (*tree)
                    .data
                    .offset((*tree).size as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            (*node).type_0 = mpack_type_map;
            return mpack_tree_parse_children(tree, node);
        }
        193 => {
            mpack_tree_flag_error(tree, mpack_error_invalid);
            return false_0 != 0;
        }
        _ => {}
    }
    if 0 as ::core::ffi::c_int == 0 {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:751\n%s\nunreachable\0" as *const u8
                as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return false_0 != 0;
}
unsafe extern "C" fn mpack_tree_parse_node(
    mut tree: *mut mpack_tree_t,
    mut node: *mut mpack_node_data_t,
) -> bool {
    if !mpack_tree_parse_node_contents(tree, node) {
        return false_0 != 0;
    }
    (*tree).parser.possible_nodes_left = (*tree)
        .parser
        .possible_nodes_left
        .wrapping_sub((*tree).parser.current_node_reserved);
    let mut node_size: size_t = (*tree)
        .parser
        .current_node_reserved
        .wrapping_add(1 as size_t);
    if (*node).type_0 as ::core::ffi::c_uint
        == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        node_size = node_size.wrapping_sub((*node).len as size_t);
    } else if (*node).type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        node_size = node_size.wrapping_sub((*node).len.wrapping_mul(2 as uint32_t) as size_t);
    }
    (*tree).size = (*tree).size.wrapping_add(node_size);
    return true_0 != 0;
}
unsafe extern "C" fn mpack_tree_continue_parsing(mut tree: *mut mpack_tree_t) -> bool {
    if mpack_tree_error(tree) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    let mut parser: *mut mpack_tree_parser_t = &raw mut (*tree).parser;
    if !((*parser).state as ::core::ffi::c_uint
        == mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:797\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"parser->state == mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    loop {
        let mut node: *mut mpack_node_data_t =
            (*(*parser).stack.offset((*parser).level as isize)).child;
        let mut level: size_t = (*parser).level;
        if !mpack_tree_parse_node(tree, node) {
            return false_0 != 0;
        }
        let ref mut fresh0 = (*(*parser).stack.offset(level as isize)).left;
        *fresh0 = (*fresh0).wrapping_sub(1);
        let ref mut fresh1 = (*(*parser).stack.offset(level as isize)).child;
        *fresh1 = (*fresh1).offset(1);
        if !(mpack_tree_error(tree) as ::core::ffi::c_uint
            == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-node.c:811\n%s\nmpack_tree_parse_node() should have returned false due to error!\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"mpack_tree_error(tree) == mpack_ok\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        while (*(*parser).stack.offset((*parser).level as isize)).left == 0 as size_t {
            if (*parser).level == 0 as size_t {
                return true_0 != 0;
            }
            (*parser).level = (*parser).level.wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn mpack_tree_cleanup(mut tree: *mut mpack_tree_t) {
    if (*tree).parser.stack_owned {
        test_free((*tree).parser.stack as *mut ::core::ffi::c_void);
        (*tree).parser.stack = ::core::ptr::null_mut::<mpack_level_t>();
        (*tree).parser.stack_owned = false_0 != 0;
    }
    let mut page: *mut mpack_tree_page_t = (*tree).next;
    while !page.is_null() {
        let mut next: *mut mpack_tree_page_t = (*page).next as *mut mpack_tree_page_t;
        test_free(page as *mut ::core::ffi::c_void);
        page = next;
    }
    (*tree).next = ::core::ptr::null_mut::<mpack_tree_page_t>();
}
unsafe extern "C" fn mpack_tree_parse_start(mut tree: *mut mpack_tree_t) -> bool {
    if mpack_tree_error(tree) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    let mut parser: *mut mpack_tree_parser_t = &raw mut (*tree).parser;
    if !((*parser).state as ::core::ffi::c_uint
        != mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:854\n%s\nprevious parsing was not finished!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"parser->state != mpack_tree_parse_state_in_progress\0" as *const u8
                as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*parser).state as ::core::ffi::c_uint
        == mpack_tree_parse_state_parsed as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_tree_cleanup(tree);
    }
    (*tree).parser.state = mpack_tree_parse_state_in_progress;
    (*tree).parser.current_node_reserved = 0 as size_t;
    if (*tree).size > 0 as size_t {
        if !(*tree).buffer.is_null() {
            memmove(
                (*tree).buffer as *mut ::core::ffi::c_void,
                (*tree).buffer.offset((*tree).size as isize) as *const ::core::ffi::c_void,
                (*tree).data_length.wrapping_sub((*tree).size),
            );
        } else {
            (*tree).data = (*tree).data.offset((*tree).size as isize);
        }
        (*tree).data_length = (*tree).data_length.wrapping_sub((*tree).size);
        (*tree).size = 0 as size_t;
        (*tree).node_count = 0 as size_t;
    }
    (*parser).possible_nodes_left = (*tree).data_length;
    if !mpack_tree_reserve_bytes(tree, ::core::mem::size_of::<uint8_t>() as size_t) {
        (*tree).parser.state = mpack_tree_parse_state_not_started;
        return false_0 != 0;
    }
    (*parser).possible_nodes_left = (*parser).possible_nodes_left.wrapping_sub(1 as size_t);
    (*tree).node_count = 1 as size_t;
    (*parser).stack = &raw mut (*parser).stack_local as *mut mpack_level_t;
    (*parser).stack_owned = false_0 != 0;
    (*parser).stack_capacity = (::core::mem::size_of::<[mpack_level_t; 3]>() as usize)
        .wrapping_div(::core::mem::size_of::<mpack_level_t>() as usize)
        as size_t;
    if (*tree).pool.is_null() {
        let mut page: *mut mpack_tree_page_t =
            test_malloc(MPACK_PAGE_ALLOC_SIZE) as *mut mpack_tree_page_t;
        if page.is_null() {
            (*tree).error = mpack_error_memory;
            return false_0 != 0;
        }
        (*page).next = ::core::ptr::null_mut::<mpack_tree_page_t>();
        (*tree).next = page;
        (*parser).nodes = &raw mut (*page).nodes as *mut mpack_node_data_t;
        (*parser).nodes_left = MPACK_NODES_PER_PAGE as size_t;
    } else {
        if (*tree).pool.is_null() {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-node.c:923\n%s\nno pool provided?\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"tree->pool != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        (*parser).nodes = (*tree).pool;
        (*parser).nodes_left = (*tree).pool_count;
    }
    (*tree).root = (*parser).nodes;
    (*parser).nodes = (*parser).nodes.offset(1);
    (*parser).nodes_left = (*parser).nodes_left.wrapping_sub(1);
    (*parser).level = 0 as size_t;
    let ref mut fresh3 = (*(*parser).stack.offset(0 as ::core::ffi::c_int as isize)).child;
    *fresh3 = (*tree).root;
    (*(*parser).stack.offset(0 as ::core::ffi::c_int as isize)).left = 1 as size_t;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_parse(mut tree: *mut mpack_tree_t) {
    if mpack_tree_error(tree) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if (*tree).parser.state as ::core::ffi::c_uint
        != mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !mpack_tree_parse_start(tree) {
            mpack_tree_flag_error(
                tree,
                (if (*tree).read_fn.is_none() {
                    mpack_error_invalid as ::core::ffi::c_int
                } else {
                    mpack_error_io as ::core::ffi::c_int
                }) as mpack_error_t,
            );
            return;
        }
    }
    if !mpack_tree_continue_parsing(tree) {
        if mpack_tree_error(tree) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
        mpack_tree_flag_error(
            tree,
            (if (*tree).read_fn.is_none() {
                mpack_error_invalid as ::core::ffi::c_int
            } else {
                mpack_error_io as ::core::ffi::c_int
            }) as mpack_error_t,
        );
        return;
    }
    if !(mpack_tree_error(tree) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:963\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"mpack_tree_error(tree) == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*tree).parser.level == 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:964\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->parser.level == 0\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*tree).parser.state = mpack_tree_parse_state_parsed;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_try_parse(mut tree: *mut mpack_tree_t) -> bool {
    if mpack_tree_error(tree) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if (*tree).parser.state as ::core::ffi::c_uint
        != mpack_tree_parse_state_in_progress as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !mpack_tree_parse_start(tree) {
            return false_0 != 0;
        }
    }
    if !mpack_tree_continue_parsing(tree) {
        return false_0 != 0;
    }
    if !(mpack_tree_error(tree) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:981\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"mpack_tree_error(tree) == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*tree).parser.level == 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:982\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"tree->parser.level == 0\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*tree).parser.state = mpack_tree_parse_state_parsed;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_root(tree: *mut mpack_tree_t) -> mpack_node_t {
    if unsafe { mpack_tree_error(tree) } as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return unsafe { mpack_tree_nil_node(tree) };
    }
    // SAFETY: FFI contract requires `tree` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let tree_ref: &mpack_tree_t = unsafe { &*tree };
    if tree_ref.parser.state as ::core::ffi::c_uint
        != mpack_tree_parse_state_parsed as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-node.c:1002\nTree has not been parsed! Did you call mpack_tree_parse() or mpack_tree_try_parse()?\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        unsafe { mpack_tree_flag_error(tree, mpack_error_bug) };
        return unsafe { mpack_tree_nil_node(tree) };
    }
    return unsafe { mpack_node(tree, tree_ref.root) };
}
unsafe extern "C" fn mpack_tree_init_clear(tree: *mut mpack_tree_t) {
    // SAFETY: FFI contract requires `tree` to be a valid, non-null,
    // aligned pointer to a correctly-sized mpack_tree_t for the duration
    // of this call.
    unsafe {
        memset(
            tree as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mpack_tree_t>() as size_t,
        );
    }
    let tree_ref: &mut mpack_tree_t = unsafe { &mut *tree };
    tree_ref.nil_node.type_0 = mpack_type_nil;
    tree_ref.missing_node.type_0 = mpack_type_missing;
    tree_ref.max_size = SIZE_MAX as size_t;
    tree_ref.max_nodes = SIZE_MAX as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_data(
    tree: *mut mpack_tree_t,
    data: *const ::core::ffi::c_char,
    length: size_t,
) {
    unsafe { mpack_tree_init_clear(tree) };
    // SAFETY: `tree` is valid per the FFI contract; mpack_tree_init_clear
    // (above) has already fully zero-initialized it before this deref.
    let tree_ref: &mut mpack_tree_t = unsafe { &mut *tree };
    tree_ref.data = data;
    tree_ref.data_length = length;
    tree_ref.pool = ::core::ptr::null_mut::<mpack_node_data_t>();
    tree_ref.pool_count = 0 as size_t;
    tree_ref.next = ::core::ptr::null_mut::<mpack_tree_page_t>();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_pool(
    mut tree: *mut mpack_tree_t,
    mut data: *const ::core::ffi::c_char,
    mut length: size_t,
    mut node_pool: *mut mpack_node_data_t,
    mut node_pool_count: size_t,
) {
    mpack_tree_init_clear(tree);
    (*tree).next = ::core::ptr::null_mut::<mpack_tree_page_t>();
    if node_pool_count == 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-node.c:1048\ninitial page has no nodes!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_tree_flag_error(tree, mpack_error_bug);
        return;
    }
    (*tree).data = data;
    (*tree).data_length = length;
    (*tree).pool = node_pool;
    (*tree).pool_count = node_pool_count;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_error(
    mut tree: *mut mpack_tree_t,
    mut error: mpack_error_t,
) {
    mpack_tree_init_clear(tree);
    (*tree).error = error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_stream(
    mut tree: *mut mpack_tree_t,
    mut read_fn: mpack_tree_read_t,
    mut context: *mut ::core::ffi::c_void,
    mut max_message_size: size_t,
    mut max_message_nodes: size_t,
) {
    mpack_tree_init_clear(tree);
    (*tree).read_fn = read_fn;
    (*tree).context = context;
    mpack_tree_set_limits(tree, max_message_size, max_message_nodes);
    (*tree).max_size = max_message_size;
    (*tree).max_nodes = max_message_nodes;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_set_limits(
    mut tree: *mut mpack_tree_t,
    mut max_message_size: size_t,
    mut max_message_nodes: size_t,
) {
    if !(max_message_size > 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1090\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"max_message_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(max_message_nodes > 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1091\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"max_message_nodes > 0\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*tree).max_size = max_message_size;
    (*tree).max_nodes = max_message_nodes;
}
unsafe extern "C" fn mpack_file_tree_teardown(mut tree: *mut mpack_tree_t) {
    let mut file_tree: *mut mpack_file_tree_t = (*tree).context as *mut mpack_file_tree_t;
    test_free((*file_tree).data as *mut ::core::ffi::c_void);
    test_free(file_tree as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn mpack_file_tree_read(
    mut tree: *mut mpack_tree_t,
    mut file_tree: *mut mpack_file_tree_t,
    mut file: *mut FILE,
    mut max_bytes: size_t,
) -> bool {
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    test_fseek(file, 0 as ::core::ffi::c_long, SEEK_END);
    error |= *__errno_location();
    let mut size: ::core::ffi::c_long = test_ftell(file);
    error |= *__errno_location();
    test_fseek(file, 0 as ::core::ffi::c_long, SEEK_SET);
    error |= *__errno_location();
    if error != 0 as ::core::ffi::c_int || size < 0 as ::core::ffi::c_long {
        mpack_tree_init_error(tree, mpack_error_io);
        return false_0 != 0;
    }
    if size == 0 as ::core::ffi::c_long {
        mpack_tree_init_error(tree, mpack_error_invalid);
        return false_0 != 0;
    }
    if max_bytes != 0 as size_t
        && (LONG_MAX as uint64_t > SIZE_MAX as uint64_t && size > SIZE_MAX as ::core::ffi::c_long
            || size as size_t > max_bytes)
    {
        mpack_tree_init_error(tree, mpack_error_too_big);
        return false_0 != 0;
    }
    (*file_tree).data = test_malloc(size as size_t) as *mut ::core::ffi::c_char;
    if (*file_tree).data.is_null() {
        mpack_tree_init_error(tree, mpack_error_memory);
        return false_0 != 0;
    }
    let mut total: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while total < size {
        let mut read: size_t = test_fread(
            (*file_tree).data.offset(total as isize) as *mut ::core::ffi::c_void,
            1 as size_t,
            (size - total) as size_t,
            file,
        );
        if read <= 0 as size_t {
            mpack_tree_init_error(tree, mpack_error_io);
            test_free((*file_tree).data as *mut ::core::ffi::c_void);
            return false_0 != 0;
        }
        total += read as ::core::ffi::c_long;
    }
    (*file_tree).size = size as size_t;
    return true_0 != 0;
}
unsafe extern "C" fn mpack_tree_file_check_max_bytes(
    mut tree: *mut mpack_tree_t,
    mut max_bytes: size_t,
) -> bool {
    if max_bytes > LONG_MAX as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-node.c:1165\nmax_bytes of %lu is invalid, maximum is LONG_MAX\0"
                as *const u8 as *const ::core::ffi::c_char,
            max_bytes as uint64_t,
        );
        mpack_tree_init_error(tree, mpack_error_bug);
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn mpack_tree_init_stdfile_noclose(
    mut tree: *mut mpack_tree_t,
    mut stdfile: *mut FILE,
    mut max_bytes: size_t,
) {
    let mut file_tree: *mut mpack_file_tree_t =
        test_malloc(::core::mem::size_of::<mpack_file_tree_t>() as size_t)
            as *mut mpack_file_tree_t;
    if file_tree.is_null() {
        mpack_tree_init_error(tree, mpack_error_memory);
        return;
    }
    if !mpack_file_tree_read(tree, file_tree, stdfile, max_bytes) {
        test_free(file_tree as *mut ::core::ffi::c_void);
        return;
    }
    mpack_tree_init_data(tree, (*file_tree).data, (*file_tree).size);
    mpack_tree_set_context(tree, file_tree as *mut ::core::ffi::c_void);
    mpack_tree_set_teardown(
        tree,
        Some(mpack_file_tree_teardown as unsafe extern "C" fn(*mut mpack_tree_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_stdfile(
    mut tree: *mut mpack_tree_t,
    mut stdfile: *mut FILE,
    mut max_bytes: size_t,
    mut close_when_done: bool,
) {
    if !mpack_tree_file_check_max_bytes(tree, max_bytes) {
        if close_when_done {
            test_fclose(stdfile);
        }
        return;
    }
    mpack_tree_init_stdfile_noclose(tree, stdfile, max_bytes);
    if close_when_done {
        test_fclose(stdfile);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_init_filename(
    mut tree: *mut mpack_tree_t,
    mut filename: *const ::core::ffi::c_char,
    mut max_bytes: size_t,
) {
    if !mpack_tree_file_check_max_bytes(tree, max_bytes) {
        return;
    }
    let mut file: *mut FILE =
        test_fopen(filename, b"rb\0" as *const u8 as *const ::core::ffi::c_char);
    if file.is_null() {
        mpack_tree_init_error(tree, mpack_error_io);
        return;
    }
    mpack_tree_init_stdfile(tree, file, max_bytes, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_destroy(mut tree: *mut mpack_tree_t) -> mpack_error_t {
    mpack_tree_cleanup(tree);
    if !(*tree).buffer.is_null() {
        test_free((*tree).buffer as *mut ::core::ffi::c_void);
    }
    if (*tree).teardown.is_some() {
        (*tree).teardown.expect("non-null function pointer")(tree);
    }
    (*tree).teardown = None;
    return (*tree).error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tree_flag_error(
    mut tree: *mut mpack_tree_t,
    mut error: mpack_error_t,
) {
    if (*tree).error as ::core::ffi::c_uint == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*tree).error = error;
        if (*tree).error_fn.is_some() {
            (*tree).error_fn.expect("non-null function pointer")(tree, error);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_flag_error(mut node: mpack_node_t, mut error: mpack_error_t) {
    mpack_tree_flag_error(node.tree, error);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_tag(mut node: mpack_node_t) -> mpack_tag_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_tag_nil();
    }
    let mut tag: mpack_tag_t = mpack_tag_t {
        type_0: mpack_type_missing,
        exttype: 0 as int8_t,
        v: C2RustUnnamed {
            u: 0 as ::core::ffi::c_int as uint64_t,
        },
    };
    tag.type_0 = (*node.data).type_0;
    match (*node.data).type_0 as ::core::ffi::c_uint {
        0 | 1 => {}
        2 => {
            tag.v.b = (*node.data).value.b;
        }
        5 => {
            tag.v.f = (*node.data).value.f;
        }
        6 => {
            tag.v.d = (*node.data).value.d;
        }
        3 => {
            tag.v.i = (*node.data).value.i;
        }
        4 => {
            tag.v.u = (*node.data).value.u;
        }
        7 => {
            tag.v.l = (*node.data).len;
        }
        8 => {
            tag.v.l = (*node.data).len;
        }
        11 => {
            tag.v.l = (*node.data).len;
            tag.exttype = mpack_node_exttype_unchecked(node);
        }
        9 => {
            tag.v.n = (*node.data).len;
        }
        10 => {
            tag.v.n = (*node.data).len;
        }
        _ => {
            if 0 as ::core::ffi::c_int == 0 {
                mpack_assert_fail_format(
                    b"mpack assertion failed at src/mpack/mpack-node.c:1291\n%s\nunrecognized type %i\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    (*node.data).type_0 as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            } else {
            };
        }
    }
    return tag;
}
unsafe extern "C" fn mpack_node_print_element(
    mut node: mpack_node_t,
    mut print: *mut mpack_print_t,
    mut depth: size_t,
) {
    let mut data: *mut mpack_node_data_t = node.data;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match (*data).type_0 as ::core::ffi::c_uint {
        7 => {
            mpack_print_append_cstr(print, b"\"\0" as *const u8 as *const ::core::ffi::c_char);
            let mut bytes: *const ::core::ffi::c_char = mpack_node_data_unchecked(node);
            i = 0 as size_t;
            while i < (*data).len as size_t {
                let mut c: ::core::ffi::c_char = *bytes.offset(i as isize);
                match c as ::core::ffi::c_int {
                    10 => {
                        mpack_print_append_cstr(
                            print,
                            b"\\n\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    92 => {
                        mpack_print_append_cstr(
                            print,
                            b"\\\\\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    34 => {
                        mpack_print_append_cstr(
                            print,
                            b"\\\"\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    _ => {
                        mpack_print_append(print, &raw mut c, 1 as size_t);
                    }
                }
                i = i.wrapping_add(1);
            }
            mpack_print_append_cstr(print, b"\"\0" as *const u8 as *const ::core::ffi::c_char);
        }
        9 => {
            mpack_print_append_cstr(print, b"[\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as size_t;
            while i < (*data).len as size_t {
                j = 0 as size_t;
                while j < depth.wrapping_add(1 as size_t) {
                    mpack_print_append_cstr(
                        print,
                        b"    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    j = j.wrapping_add(1);
                }
                mpack_node_print_element(
                    mpack_node_array_at(node, i),
                    print,
                    depth.wrapping_add(1 as size_t),
                );
                if i != (*data).len.wrapping_sub(1 as uint32_t) as size_t {
                    mpack_print_append_cstr(
                        print,
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                mpack_print_append_cstr(print, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < depth {
                mpack_print_append_cstr(
                    print,
                    b"    \0" as *const u8 as *const ::core::ffi::c_char,
                );
                i = i.wrapping_add(1);
            }
            mpack_print_append_cstr(print, b"]\0" as *const u8 as *const ::core::ffi::c_char);
        }
        10 => {
            mpack_print_append_cstr(print, b"{\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as size_t;
            while i < (*data).len as size_t {
                j = 0 as size_t;
                while j < depth.wrapping_add(1 as size_t) {
                    mpack_print_append_cstr(
                        print,
                        b"    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    j = j.wrapping_add(1);
                }
                mpack_node_print_element(
                    mpack_node_map_key_at(node, i),
                    print,
                    depth.wrapping_add(1 as size_t),
                );
                mpack_print_append_cstr(print, b": \0" as *const u8 as *const ::core::ffi::c_char);
                mpack_node_print_element(
                    mpack_node_map_value_at(node, i),
                    print,
                    depth.wrapping_add(1 as size_t),
                );
                if i != (*data).len.wrapping_sub(1 as uint32_t) as size_t {
                    mpack_print_append_cstr(
                        print,
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                mpack_print_append_cstr(print, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < depth {
                mpack_print_append_cstr(
                    print,
                    b"    \0" as *const u8 as *const ::core::ffi::c_char,
                );
                i = i.wrapping_add(1);
            }
            mpack_print_append_cstr(print, b"}\0" as *const u8 as *const ::core::ffi::c_char);
        }
        _ => {
            let mut prefix: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut prefix_length: size_t = 0 as size_t;
            if mpack_node_type(node) as ::core::ffi::c_uint
                == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
                || mpack_node_type(node) as ::core::ffi::c_uint
                    == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                prefix = mpack_node_data(node);
                prefix_length = mpack_node_data_len(node) as size_t;
            }
            let mut buf: [::core::ffi::c_char; 256] = [0; 256];
            let mut tag: mpack_tag_t = mpack_node_tag(node);
            mpack_tag_debug_pseudo_json(
                tag,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                prefix,
                prefix_length,
            );
            mpack_print_append_cstr(print, &raw mut buf as *mut ::core::ffi::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_print_to_buffer(
    mut node: mpack_node_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) {
    if buffer_size == 0 as size_t {
        if 0 as ::core::ffi::c_int == 0 {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-node.c:1375\n%s\nbuffer size is zero!\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        return;
    }
    let mut print: mpack_print_t = mpack_print_t {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        count: 0,
        callback: None,
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut print as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_print_t>() as size_t,
    );
    print.buffer = buffer;
    print.size = buffer_size;
    mpack_node_print_element(node, &raw mut print, 0 as size_t);
    mpack_print_append(
        &raw mut print,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        1 as size_t,
    );
    mpack_print_flush(&raw mut print);
    *print
        .buffer
        .offset(print.size.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_print_to_callback(
    mut node: mpack_node_t,
    mut callback: mpack_print_callback_t,
    mut context: *mut ::core::ffi::c_void,
) {
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut print: mpack_print_t = mpack_print_t {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        count: 0,
        callback: None,
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut print as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_print_t>() as size_t,
    );
    print.buffer = &raw mut buffer as *mut ::core::ffi::c_char;
    print.size = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize as size_t;
    print.callback = callback;
    print.context = context;
    mpack_node_print_element(node, &raw mut print, 0 as size_t);
    mpack_print_flush(&raw mut print);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_print_to_file(mut node: mpack_node_t, mut file: *mut FILE) {
    if file.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1405\n%s\nfile is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"file != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut print: mpack_print_t = mpack_print_t {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        count: 0,
        callback: None,
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut print as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_print_t>() as size_t,
    );
    print.buffer = &raw mut buffer as *mut ::core::ffi::c_char;
    print.size = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize as size_t;
    print.callback = Some(
        mpack_print_file_callback
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                size_t,
            ) -> (),
    ) as mpack_print_callback_t;
    print.context = file as *mut ::core::ffi::c_void;
    let mut depth: size_t = 2 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < depth {
        mpack_print_append_cstr(
            &raw mut print,
            b"    \0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = i.wrapping_add(1);
    }
    mpack_node_print_element(node, &raw mut print, depth);
    mpack_print_append_cstr(
        &raw mut print,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    mpack_print_flush(&raw mut print);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_timestamp(mut node: mpack_node_t) -> mpack_timestamp_t {
    let mut timestamp: mpack_timestamp_t = mpack_timestamp_t {
        seconds: 0 as int64_t,
        nanoseconds: 0 as uint32_t,
    };
    if mpack_node_exttype(node) as ::core::ffi::c_int
        != MPACK_EXTTYPE_TIMESTAMP as ::core::ffi::c_int
    {
        mpack_node_flag_error(node, mpack_error_type);
        return timestamp;
    }
    let mut p: *const ::core::ffi::c_char = mpack_node_data_unchecked(node);
    match (*node.data).len {
        4 => {
            timestamp.nanoseconds = 0 as uint32_t;
            timestamp.seconds = mpack_load_u32(p) as int64_t;
        }
        8 => {
            let mut value: uint64_t = mpack_load_u64(p);
            timestamp.nanoseconds = (value >> 34 as ::core::ffi::c_int) as uint32_t;
            timestamp.seconds = (value
                & ((1 as uint64_t) << 34 as ::core::ffi::c_int).wrapping_sub(1 as uint64_t))
                as int64_t;
        }
        12 => {
            timestamp.nanoseconds = mpack_load_u32(p);
            timestamp.seconds = mpack_load_i64(p.offset(4 as ::core::ffi::c_int as isize));
        }
        _ => {
            mpack_tree_flag_error(node.tree, mpack_error_invalid);
            return timestamp;
        }
    }
    if timestamp.nanoseconds > MPACK_TIMESTAMP_NANOSECONDS_MAX as uint32_t {
        mpack_tree_flag_error(node.tree, mpack_error_invalid);
        let mut zero: mpack_timestamp_t = mpack_timestamp_t {
            seconds: 0 as int64_t,
            nanoseconds: 0 as uint32_t,
        };
        return zero;
    }
    return timestamp;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_timestamp_seconds(mut node: mpack_node_t) -> int64_t {
    return mpack_node_timestamp(node).seconds;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_timestamp_nanoseconds(mut node: mpack_node_t) -> uint32_t {
    return mpack_node_timestamp(node).nanoseconds;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_check_utf8(mut node: mpack_node_t) {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut data: *mut mpack_node_data_t = node.data;
    if (*data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || !mpack_utf8_check(mpack_node_data_unchecked(node), (*data).len as size_t)
    {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_check_utf8_cstr(mut node: mpack_node_t) {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut data: *mut mpack_node_data_t = node.data;
    if (*data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || !mpack_utf8_check_no_null(mpack_node_data_unchecked(node), (*data).len as size_t)
    {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_copy_data(
    mut node: mpack_node_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if !(bufsize == 0 as size_t || !buffer.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1511\n%s\nbuffer is NULL for maximum of %i bytes\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"bufsize == 0 || buffer != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            bufsize as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        && type_0 as ::core::ffi::c_uint
            != mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        && type_0 as ::core::ffi::c_uint
            != mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return 0 as size_t;
    }
    if (*node.data).len as size_t > bufsize {
        mpack_node_flag_error(node, mpack_error_too_big);
        return 0 as size_t;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    return (*node.data).len as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_copy_utf8(
    mut node: mpack_node_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if !(bufsize == 0 as size_t || !buffer.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1536\n%s\nbuffer is NULL for maximum of %i bytes\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"bufsize == 0 || buffer != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            bufsize as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return 0 as size_t;
    }
    if (*node.data).len as size_t > bufsize {
        mpack_node_flag_error(node, mpack_error_too_big);
        return 0 as size_t;
    }
    if !mpack_utf8_check(mpack_node_data_unchecked(node), (*node.data).len as size_t) {
        mpack_node_flag_error(node, mpack_error_type);
        return 0 as size_t;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    return (*node.data).len as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_copy_cstr(
    mut node: mpack_node_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) {
    if buffer.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1562\n%s\nbuffer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(bufsize >= 1 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1563\n%s\nbuffer size is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"bufsize >= 1\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        return;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_type);
        return;
    }
    if (*node.data).len as size_t > bufsize.wrapping_sub(1 as size_t) {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_too_big);
        return;
    }
    if !mpack_str_check_no_null(mpack_node_data_unchecked(node), (*node.data).len as size_t) {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_type);
        return;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    *buffer.offset((*node.data).len as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_copy_utf8_cstr(
    mut node: mpack_node_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) {
    if buffer.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1596\n%s\nbuffer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(bufsize >= 1 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1597\n%s\nbuffer size is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"bufsize >= 1\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        return;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_type);
        return;
    }
    if (*node.data).len as size_t > bufsize.wrapping_sub(1 as size_t) {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_too_big);
        return;
    }
    if !mpack_utf8_check_no_null(mpack_node_data_unchecked(node), (*node.data).len as size_t) {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        mpack_node_flag_error(node, mpack_error_type);
        return;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    *buffer.offset((*node.data).len as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_data_alloc(
    mut node: mpack_node_t,
    mut maxlen: size_t,
) -> *mut ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        && type_0 as ::core::ffi::c_uint
            != mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        && type_0 as ::core::ffi::c_uint
            != mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*node.data).len as size_t > maxlen {
        mpack_node_flag_error(node, mpack_error_too_big);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut ret: *mut ::core::ffi::c_char =
        test_malloc((*node.data).len as size_t) as *mut ::core::ffi::c_char;
    if ret.is_null() {
        mpack_node_flag_error(node, mpack_error_memory);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_cstr_alloc(
    mut node: mpack_node_t,
    mut maxlen: size_t,
) -> *mut ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if maxlen < 1 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-node.c:1663\nmaxlen is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_node_flag_error(node, mpack_error_bug);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*node.data).len as size_t > maxlen.wrapping_sub(1 as size_t) {
        mpack_node_flag_error(node, mpack_error_too_big);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !mpack_str_check_no_null(mpack_node_data_unchecked(node), (*node.data).len as size_t) {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut ret: *mut ::core::ffi::c_char =
        test_malloc((*node.data).len.wrapping_add(1 as uint32_t) as size_t)
            as *mut ::core::ffi::c_char;
    if ret.is_null() {
        mpack_node_flag_error(node, mpack_error_memory);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    *ret.offset((*node.data).len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_utf8_cstr_alloc(
    mut node: mpack_node_t,
    mut maxlen: size_t,
) -> *mut ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if maxlen < 1 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-node.c:1700\nmaxlen is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_node_flag_error(node, mpack_error_bug);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*node.data).len as size_t > maxlen.wrapping_sub(1 as size_t) {
        mpack_node_flag_error(node, mpack_error_too_big);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !mpack_utf8_check_no_null(mpack_node_data_unchecked(node), (*node.data).len as size_t) {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut ret: *mut ::core::ffi::c_char =
        test_malloc((*node.data).len.wrapping_add(1 as uint32_t) as size_t)
            as *mut ::core::ffi::c_char;
    if ret.is_null() {
        mpack_node_flag_error(node, mpack_error_memory);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        mpack_node_data_unchecked(node) as *const ::core::ffi::c_void,
        (*node.data).len as size_t,
    );
    *ret.offset((*node.data).len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return ret;
}
unsafe extern "C" fn mpack_node_map_int_impl(
    mut node: mpack_node_t,
    mut num: int64_t,
) -> *mut mpack_node_data_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    let mut found: *mut mpack_node_data_t = ::core::ptr::null_mut::<mpack_node_data_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*node.data).len as size_t {
        let mut key: *mut mpack_node_data_t = mpack_node_child(node, i.wrapping_mul(2 as size_t));
        if (*key).type_0 as ::core::ffi::c_uint
            == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*key).value.i == num
            || (*key).type_0 as ::core::ffi::c_uint
                == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
                && num >= 0 as int64_t
                && (*key).value.u == num as uint64_t
        {
            if !found.is_null() {
                mpack_node_flag_error(node, mpack_error_data);
                return ::core::ptr::null_mut::<mpack_node_data_t>();
            }
            found = mpack_node_child(node, i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t));
        }
        i = i.wrapping_add(1);
    }
    if !found.is_null() {
        return found;
    }
    return ::core::ptr::null_mut::<mpack_node_data_t>();
}
unsafe extern "C" fn mpack_node_map_uint_impl(
    mut node: mpack_node_t,
    mut num: uint64_t,
) -> *mut mpack_node_data_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    let mut found: *mut mpack_node_data_t = ::core::ptr::null_mut::<mpack_node_data_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*node.data).len as size_t {
        let mut key: *mut mpack_node_data_t = mpack_node_child(node, i.wrapping_mul(2 as size_t));
        if (*key).type_0 as ::core::ffi::c_uint
            == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*key).value.u == num
            || (*key).type_0 as ::core::ffi::c_uint
                == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*key).value.i >= 0 as int64_t
                && (*key).value.i as uint64_t == num
        {
            if !found.is_null() {
                mpack_node_flag_error(node, mpack_error_data);
                return ::core::ptr::null_mut::<mpack_node_data_t>();
            }
            found = mpack_node_child(node, i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t));
        }
        i = i.wrapping_add(1);
    }
    if !found.is_null() {
        return found;
    }
    return ::core::ptr::null_mut::<mpack_node_data_t>();
}
unsafe extern "C" fn mpack_node_map_str_impl(
    mut node: mpack_node_t,
    mut str: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut mpack_node_data_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    if !(length == 0 as size_t || !str.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1805\n%s\nstr of length %i is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"length == 0 || str != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            length as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return ::core::ptr::null_mut::<mpack_node_data_t>();
    }
    let mut tree: *mut mpack_tree_t = node.tree;
    let mut found: *mut mpack_node_data_t = ::core::ptr::null_mut::<mpack_node_data_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*node.data).len as size_t {
        let mut key: *mut mpack_node_data_t = mpack_node_child(node, i.wrapping_mul(2 as size_t));
        if (*key).type_0 as ::core::ffi::c_uint
            == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*key).len as size_t == length
            && memcmp(
                str as *const ::core::ffi::c_void,
                mpack_node_data_unchecked(mpack_node(tree, key)) as *const ::core::ffi::c_void,
                length,
            ) == 0 as ::core::ffi::c_int
        {
            if !found.is_null() {
                mpack_node_flag_error(node, mpack_error_data);
                return ::core::ptr::null_mut::<mpack_node_data_t>();
            }
            found = mpack_node_child(node, i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t));
        }
        i = i.wrapping_add(1);
    }
    if !found.is_null() {
        return found;
    }
    return ::core::ptr::null_mut::<mpack_node_data_t>();
}
unsafe extern "C" fn mpack_node_wrap_lookup(
    mut tree: *mut mpack_tree_t,
    mut data: *mut mpack_node_data_t,
) -> mpack_node_t {
    if data.is_null() {
        if (*tree).error as ::core::ffi::c_uint
            == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            mpack_tree_flag_error(tree, mpack_error_data);
        }
        return mpack_tree_nil_node(tree);
    }
    return mpack_node(tree, data);
}
unsafe extern "C" fn mpack_node_wrap_lookup_optional(
    mut tree: *mut mpack_tree_t,
    mut data: *mut mpack_node_data_t,
) -> mpack_node_t {
    if data.is_null() {
        if (*tree).error as ::core::ffi::c_uint
            == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return mpack_tree_missing_node(tree);
        }
        return mpack_tree_nil_node(tree);
    }
    return mpack_node(tree, data);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_int(
    mut node: mpack_node_t,
    mut num: int64_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup(node.tree, mpack_node_map_int_impl(node, num));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_int_optional(
    mut node: mpack_node_t,
    mut num: int64_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup_optional(node.tree, mpack_node_map_int_impl(node, num));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_uint(
    mut node: mpack_node_t,
    mut num: uint64_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup(node.tree, mpack_node_map_uint_impl(node, num));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_uint_optional(
    mut node: mpack_node_t,
    mut num: uint64_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup_optional(node.tree, mpack_node_map_uint_impl(node, num));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_str(
    mut node: mpack_node_t,
    mut str: *const ::core::ffi::c_char,
    mut length: size_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup(node.tree, mpack_node_map_str_impl(node, str, length));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_str_optional(
    mut node: mpack_node_t,
    mut str: *const ::core::ffi::c_char,
    mut length: size_t,
) -> mpack_node_t {
    return mpack_node_wrap_lookup_optional(node.tree, mpack_node_map_str_impl(node, str, length));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_cstr(
    mut node: mpack_node_t,
    mut cstr: *const ::core::ffi::c_char,
) -> mpack_node_t {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1878\n%s\ncstr is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return mpack_node_map_str(node, cstr, test_strlen(cstr));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_cstr_optional(
    mut node: mpack_node_t,
    mut cstr: *const ::core::ffi::c_char,
) -> mpack_node_t {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1883\n%s\ncstr is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return mpack_node_map_str_optional(node, cstr, test_strlen(cstr));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_contains_int(
    mut node: mpack_node_t,
    mut num: int64_t,
) -> bool {
    return !mpack_node_map_int_impl(node, num).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_contains_uint(
    mut node: mpack_node_t,
    mut num: uint64_t,
) -> bool {
    return !mpack_node_map_uint_impl(node, num).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_contains_str(
    mut node: mpack_node_t,
    mut str: *const ::core::ffi::c_char,
    mut length: size_t,
) -> bool {
    return !mpack_node_map_str_impl(node, str, length).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_contains_cstr(
    mut node: mpack_node_t,
    mut cstr: *const ::core::ffi::c_char,
) -> bool {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1900\n%s\ncstr is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return mpack_node_map_contains_str(node, cstr, test_strlen(cstr));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_enum_optional(
    mut node: mpack_node_t,
    mut strings: *mut *const ::core::ffi::c_char,
    mut count: size_t,
) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
    if mpack_node_type(node) as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
    let mut key: *const ::core::ffi::c_char = mpack_node_str(node);
    let mut keylen: size_t = mpack_node_strlen(node);
    if !(mpack_node_error(node) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-node.c:1915\n%s\nthese should not fail\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"mpack_node_error(node) == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < count {
        let mut other: *const ::core::ffi::c_char = *strings.offset(i as isize);
        let mut otherlen: size_t = test_strlen(other);
        if keylen == otherlen
            && memcmp(
                key as *const ::core::ffi::c_void,
                other as *const ::core::ffi::c_void,
                keylen,
            ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i = i.wrapping_add(1);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_enum(
    mut node: mpack_node_t,
    mut strings: *mut *const ::core::ffi::c_char,
    mut count: size_t,
) -> size_t {
    let mut value: size_t = mpack_node_enum_optional(node, strings, count);
    if value == count {
        mpack_node_flag_error(node, mpack_error_type);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_type(node: mpack_node_t) -> mpack_type_t {
    if unsafe { mpack_node_error(node) } as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_type_nil;
    }
    // SAFETY: FFI contract requires `node.data` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let data_ref = unsafe { &*node.data };
    return data_ref.type_0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_is_nil(node: mpack_node_t) -> bool {
    if unsafe { mpack_node_error(node) } as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    // SAFETY: FFI contract requires `node.data` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let data_ref = unsafe { &*node.data };
    return data_ref.type_0 as ::core::ffi::c_uint
        == mpack_type_nil as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_is_missing(mut node: mpack_node_t) -> bool {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    return (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_missing as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_nil(mut node: mpack_node_t) {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_nil as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_missing(mut node: mpack_node_t) {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_missing as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_bool(mut node: mpack_node_t) -> bool {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_bool as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.b;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_true(mut node: mpack_node_t) {
    if mpack_node_bool(node) as ::core::ffi::c_int != true_0 {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_false(mut node: mpack_node_t) {
    if mpack_node_bool(node) as ::core::ffi::c_int != false_0 {
        mpack_node_flag_error(node, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_u8(mut node: mpack_node_t) -> uint8_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint8_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_UINT8_MAX as uint64_t {
            return (*node.data).value.u as uint8_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= 0 as int64_t
            && (*node.data).value.i <= MPACK_UINT8_MAX as int64_t
        {
            return (*node.data).value.i as uint8_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_i8(mut node: mpack_node_t) -> int8_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int8_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_INT8_MAX as uint64_t {
            return (*node.data).value.u as int8_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= MPACK_INT8_MIN as int64_t
            && (*node.data).value.i <= MPACK_INT8_MAX as int64_t
        {
            return (*node.data).value.i as int8_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_u16(mut node: mpack_node_t) -> uint16_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint16_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_UINT16_MAX as uint64_t {
            return (*node.data).value.u as uint16_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= 0 as int64_t
            && (*node.data).value.i <= MPACK_UINT16_MAX as int64_t
        {
            return (*node.data).value.i as uint16_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_i16(mut node: mpack_node_t) -> int16_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int16_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_INT16_MAX as uint64_t {
            return (*node.data).value.u as int16_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= MPACK_INT16_MIN as int64_t
            && (*node.data).value.i <= MPACK_INT16_MAX as int64_t
        {
            return (*node.data).value.i as int16_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_u32(mut node: mpack_node_t) -> uint32_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_UINT32_MAX as uint64_t {
            return (*node.data).value.u as uint32_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= 0 as int64_t
            && (*node.data).value.i <= MPACK_UINT32_MAX as int64_t
        {
            return (*node.data).value.i as uint32_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_i32(mut node: mpack_node_t) -> int32_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int32_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_INT32_MAX as uint64_t {
            return (*node.data).value.u as int32_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= MPACK_INT32_MIN as int64_t
            && (*node.data).value.i <= MPACK_INT32_MAX as int64_t
        {
            return (*node.data).value.i as int32_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_u64(mut node: mpack_node_t) -> uint64_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint64_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.u;
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.i >= 0 as int64_t {
            return (*node.data).value.i as uint64_t;
        }
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_i64(mut node: mpack_node_t) -> int64_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int64_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*node.data).value.u <= MPACK_INT64_MAX as uint64_t {
            return (*node.data).value.u as int64_t;
        }
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.i;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_uint(mut node: mpack_node_t) -> ::core::ffi::c_uint {
    if ::core::mem::size_of::<::core::ffi::c_uint>() as usize == 4 as usize {
        return mpack_node_u32(node) as ::core::ffi::c_uint;
    }
    let mut val: uint64_t = mpack_node_u64(node);
    if val <= MPACK_UINT_MAX as uint64_t {
        return val as ::core::ffi::c_uint;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_int(mut node: mpack_node_t) -> ::core::ffi::c_int {
    if ::core::mem::size_of::<::core::ffi::c_int>() as usize == 4 as usize {
        return mpack_node_i32(node) as ::core::ffi::c_int;
    }
    let mut val: int64_t = mpack_node_i64(node);
    if val >= MPACK_INT_MIN as int64_t && val <= MPACK_INT_MAX as int64_t {
        return val as ::core::ffi::c_int;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_float(mut node: mpack_node_t) -> ::core::ffi::c_float {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f32;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.u as ::core::ffi::c_float;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.i as ::core::ffi::c_float;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.f;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.d as ::core::ffi::c_float;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_double(mut node: mpack_node_t) -> ::core::ffi::c_double {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.u as ::core::ffi::c_double;
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.i as ::core::ffi::c_double;
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.f as ::core::ffi::c_double;
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.d;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_float_strict(mut node: mpack_node_t) -> ::core::ffi::c_float {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f32;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.f;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_double_strict(mut node: mpack_node_t) -> ::core::ffi::c_double {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.f as ::core::ffi::c_double;
    } else if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).value.d;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_exttype(mut node: mpack_node_t) -> int8_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int8_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_node_exttype_unchecked(node);
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_data_len(mut node: mpack_node_t) -> uint32_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).len;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_strlen(mut node: mpack_node_t) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).len as size_t;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_str(mut node: mpack_node_t) -> *const ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_node_data_unchecked(node);
    }
    mpack_node_flag_error(node, mpack_error_type);
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_data(mut node: mpack_node_t) -> *const ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut type_0: mpack_type_t = (*node.data).type_0;
    if type_0 as ::core::ffi::c_uint == mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_node_data_unchecked(node);
    }
    mpack_node_flag_error(node, mpack_error_type);
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_bin_data(mut node: mpack_node_t) -> *const ::core::ffi::c_char {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_node_data_unchecked(node);
    }
    mpack_node_flag_error(node, mpack_error_type);
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_bin_size(mut node: mpack_node_t) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*node.data).len as size_t;
    }
    mpack_node_flag_error(node, mpack_error_type);
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_array_length(mut node: mpack_node_t) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return 0 as size_t;
    }
    return (*node.data).len as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_array_at(
    mut node: mpack_node_t,
    mut index: size_t,
) -> mpack_node_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_tree_nil_node(node.tree);
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return mpack_tree_nil_node(node.tree);
    }
    if index >= (*node.data).len as size_t {
        mpack_node_flag_error(node, mpack_error_data);
        return mpack_tree_nil_node(node.tree);
    }
    return mpack_node(node.tree, mpack_node_child(node, index));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_count(mut node: mpack_node_t) -> size_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return 0 as size_t;
    }
    return (*node.data).len as size_t;
}
unsafe extern "C" fn mpack_node_map_at(
    mut node: mpack_node_t,
    mut index: size_t,
    mut offset: size_t,
) -> mpack_node_t {
    if mpack_node_error(node) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_tree_nil_node(node.tree);
    }
    if (*node.data).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_node_flag_error(node, mpack_error_type);
        return mpack_tree_nil_node(node.tree);
    }
    if index >= (*node.data).len as size_t {
        mpack_node_flag_error(node, mpack_error_data);
        return mpack_tree_nil_node(node.tree);
    }
    return mpack_node(
        node.tree,
        mpack_node_child(node, index.wrapping_mul(2 as size_t).wrapping_add(offset)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_key_at(
    mut node: mpack_node_t,
    mut index: size_t,
) -> mpack_node_t {
    return mpack_node_map_at(node, index, 0 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_node_map_value_at(
    mut node: mpack_node_t,
    mut index: size_t,
) -> mpack_node_t {
    return mpack_node_map_at(node, index, 1 as size_t);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const MPACK_BUFFER_SIZE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MPACK_NODE_PAGE_SIZE: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
