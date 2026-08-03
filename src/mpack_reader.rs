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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn test_ferror(stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn mpack_assert_fail_format(format: *const ::core::ffi::c_char, ...) -> !;
    fn mpack_break_hit_format(format: *const ::core::ffi::c_char, ...);
    fn mpack_error_to_string(error: mpack_error_t) -> *const ::core::ffi::c_char;
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
    fn mpack_track_init(track: *mut mpack_track_t) -> mpack_error_t;
    fn mpack_track_push(
        track: *mut mpack_track_t,
        type_0: mpack_type_t,
        count: uint32_t,
    ) -> mpack_error_t;
    fn mpack_track_pop(track: *mut mpack_track_t, type_0: mpack_type_t) -> mpack_error_t;
    fn mpack_track_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_peek_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_bytes(track: *mut mpack_track_t, read: bool, count: size_t) -> mpack_error_t;
    fn mpack_track_str_bytes_all(
        track: *mut mpack_track_t,
        read: bool,
        count: size_t,
    ) -> mpack_error_t;
    fn mpack_track_check_empty(track: *mut mpack_track_t) -> mpack_error_t;
    fn mpack_track_destroy(track: *mut mpack_track_t, cancel: bool) -> mpack_error_t;
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
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MPACK_MAXIMUM_TAG_SIZE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn mpack_tag_make_bool(mut value: bool) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_int(mut value: int64_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_uint(mut value: uint64_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_float(mut value: ::core::ffi::c_float) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_double(mut value: ::core::ffi::c_double) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_array(mut count: uint32_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_map(mut count: uint32_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_str(mut length: uint32_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_bin(mut length: uint32_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_make_ext(mut exttype: int8_t, mut length: uint32_t) -> mpack_tag_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_bin_length(mut tag: *mut mpack_tag_t) -> uint32_t {
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
#[inline]
unsafe extern "C" fn mpack_tag_ext_length(mut tag: *mut mpack_tag_t) -> uint32_t {
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
pub const MPACK_TAG_SIZE_U8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_U16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_U32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_U64: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_I8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_I16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_I32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_I64: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FLOAT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_DOUBLE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_ARRAY16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_ARRAY32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_MAP16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_MAP32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_STR8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_STR16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_STR32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_BIN8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_BIN16: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_BIN32: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT16: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_EXT8: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_EXT16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_EXT32: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MPACK_READER_SMALL_FRACTION_DENOMINATOR: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MPACK_READER_MINIMUM_BUFFER_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mpack_reader_set_context(
    mut reader: *mut mpack_reader_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*reader).context = context;
}
#[inline]
unsafe extern "C" fn mpack_reader_set_teardown(
    mut reader: *mut mpack_reader_t,
    mut teardown: mpack_reader_teardown_t,
) {
    (*reader).teardown = teardown;
}
#[inline]
unsafe extern "C" fn mpack_reader_error(mut reader: *mut mpack_reader_t) -> mpack_error_t {
    return (*reader).error;
}
#[inline]
unsafe extern "C" fn mpack_reader_flag_if_error(
    mut reader: *mut mpack_reader_t,
    mut error: mpack_error_t,
) -> mpack_error_t {
    if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        mpack_reader_flag_error(reader, error);
    }
    return mpack_reader_error(reader);
}
#[inline]
unsafe extern "C" fn mpack_done_array(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_array);
}
#[inline]
unsafe extern "C" fn mpack_done_map(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_map);
}
#[inline]
unsafe extern "C" fn mpack_done_str(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_str);
}
#[inline]
unsafe extern "C" fn mpack_done_bin(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_bin);
}
#[inline]
unsafe extern "C" fn mpack_done_ext(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_ext);
}
#[inline]
unsafe extern "C" fn mpack_reader_ensure(
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
#[inline]
unsafe extern "C" fn mpack_read_native(
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
#[inline]
unsafe extern "C" fn mpack_reader_track_element(mut reader: *mut mpack_reader_t) -> mpack_error_t {
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
#[inline]
unsafe extern "C" fn mpack_reader_track_peek_element(
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
#[inline]
unsafe extern "C" fn mpack_reader_track_bytes(
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
#[inline]
unsafe extern "C" fn mpack_reader_track_str_bytes_all(
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
pub unsafe extern "C" fn mpack_reader_init(
    mut reader: *mut mpack_reader_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut count: size_t,
) {
    if buffer.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:33\n%s\nbuffer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    memset(
        reader as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_reader_t>() as size_t,
    );
    (*reader).buffer = buffer;
    (*reader).size = size;
    (*reader).data = buffer;
    (*reader).end = buffer.offset(count as isize);
    mpack_reader_flag_if_error(reader, mpack_track_init(&raw mut (*reader).track));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_init_error(
    mut reader: *mut mpack_reader_t,
    mut error: mpack_error_t,
) {
    memset(
        reader as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_reader_t>() as size_t,
    );
    (*reader).error = error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_init_data(
    mut reader: *mut mpack_reader_t,
    mut data: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !data.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:58\n%s\ndata is NULL for %zu bytes\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || data != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    memset(
        reader as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_reader_t>() as size_t,
    );
    (*reader).data = data;
    (*reader).end = data.offset(count as isize);
    mpack_reader_flag_if_error(reader, mpack_track_init(&raw mut (*reader).track));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_set_fill(
    mut reader: *mut mpack_reader_t,
    mut fill: mpack_reader_fill_t,
) {
    if (*reader).size == 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-reader.c:77\ncannot use fill function without a writeable buffer!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_reader_flag_error(reader, mpack_error_bug);
        return;
    }
    if (*reader).size < MPACK_READER_MINIMUM_BUFFER_SIZE as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-reader.c:84\nbuffer size is %i, but minimum buffer size for fill is %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*reader).size as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
        );
        mpack_reader_flag_error(reader, mpack_error_bug);
        return;
    }
    (*reader).fill = fill;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_set_skip(
    mut reader: *mut mpack_reader_t,
    mut skip: mpack_reader_skip_t,
) {
    if !((*reader).size != 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:93\n%s\ncannot use skip function without a writeable buffer!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->size != 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*reader).skip = skip;
}
unsafe extern "C" fn mpack_file_reader_fill(
    mut reader: *mut mpack_reader_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut count: size_t,
) -> size_t {
    if feof((*reader).context as *mut FILE) != 0 {
        mpack_reader_flag_error(reader, mpack_error_eof);
        return 0 as size_t;
    }
    return test_fread(
        buffer as *mut ::core::ffi::c_void,
        1 as size_t,
        count,
        (*reader).context as *mut FILE,
    );
}
unsafe extern "C" fn mpack_file_reader_skip(mut reader: *mut mpack_reader_t, mut count: size_t) {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut file: *mut FILE = (*reader).context as *mut FILE;
    if test_ftell(file) >= 0 as ::core::ffi::c_long {
        if test_fseek(file, count as ::core::ffi::c_long, SEEK_CUR) == 0 as ::core::ffi::c_int {
            return;
        }
        if test_ferror(file) != 0 {
            mpack_reader_flag_error(reader, mpack_error_io);
            return;
        }
    }
    mpack_reader_skip_using_fill(reader, count);
}
unsafe extern "C" fn mpack_file_reader_teardown(mut reader: *mut mpack_reader_t) {
    test_free((*reader).buffer as *mut ::core::ffi::c_void);
    (*reader).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*reader).context = NULL;
    (*reader).size = 0 as size_t;
    (*reader).fill = None;
    (*reader).skip = None;
    (*reader).teardown = None;
}
unsafe extern "C" fn mpack_file_reader_teardown_close(mut reader: *mut mpack_reader_t) {
    let mut file: *mut FILE = (*reader).context as *mut FILE;
    if !file.is_null() {
        let mut ret: ::core::ffi::c_int = test_fclose(file);
        if ret != 0 as ::core::ffi::c_int {
            mpack_reader_flag_error(reader, mpack_error_io);
        }
    }
    mpack_file_reader_teardown(reader);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_init_stdfile(
    mut reader: *mut mpack_reader_t,
    mut file: *mut FILE,
    mut close_when_done: bool,
) {
    if file.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:151\n%s\nfile is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"file != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut capacity: size_t = MPACK_BUFFER_SIZE as size_t;
    let mut buffer: *mut ::core::ffi::c_char = test_malloc(capacity) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        mpack_reader_init_error(reader, mpack_error_memory);
        if close_when_done {
            test_fclose(file);
        }
        return;
    }
    mpack_reader_init(reader, buffer, capacity, 0 as size_t);
    mpack_reader_set_context(reader, file as *mut ::core::ffi::c_void);
    mpack_reader_set_fill(
        reader,
        Some(
            mpack_file_reader_fill
                as unsafe extern "C" fn(
                    *mut mpack_reader_t,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> size_t,
        ),
    );
    mpack_reader_set_skip(
        reader,
        Some(mpack_file_reader_skip as unsafe extern "C" fn(*mut mpack_reader_t, size_t) -> ()),
    );
    mpack_reader_set_teardown(
        reader,
        if close_when_done as ::core::ffi::c_int != 0 {
            Some(
                mpack_file_reader_teardown_close as unsafe extern "C" fn(*mut mpack_reader_t) -> (),
            )
        } else {
            Some(mpack_file_reader_teardown as unsafe extern "C" fn(*mut mpack_reader_t) -> ())
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_init_filename(
    mut reader: *mut mpack_reader_t,
    mut filename: *const ::core::ffi::c_char,
) {
    if filename.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:173\n%s\nfilename is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"filename != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut file: *mut FILE =
        test_fopen(filename, b"rb\0" as *const u8 as *const ::core::ffi::c_char);
    if file.is_null() {
        mpack_reader_init_error(reader, mpack_error_io);
        return;
    }
    mpack_reader_init_stdfile(reader, file, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_destroy(mut reader: *mut mpack_reader_t) -> mpack_error_t {
    mpack_reader_flag_if_error(
        reader,
        mpack_track_destroy(
            &raw mut (*reader).track,
            mpack_reader_error(reader) as ::core::ffi::c_uint
                != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    if (*reader).teardown.is_some() {
        (*reader).teardown.expect("non-null function pointer")(reader);
    }
    (*reader).teardown = None;
    return (*reader).error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_remaining(
    mut reader: *mut mpack_reader_t,
    mut data: *mut *const ::core::ffi::c_char,
) -> size_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if mpack_reader_flag_if_error(reader, mpack_track_check_empty(&raw mut (*reader).track))
        as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    if !data.is_null() {
        *data = (*reader).data;
    }
    return (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_reader_flag_error(
    mut reader: *mut mpack_reader_t,
    mut error: mpack_error_t,
) {
    if (*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*reader).error = error;
        (*reader).end = (*reader).data;
        if (*reader).error_fn.is_some() {
            (*reader).error_fn.expect("non-null function pointer")(reader, error);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn mpack_fill_range(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut min_bytes: size_t,
    mut max_bytes: size_t,
) -> size_t {
    if (*reader).fill.is_none() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:227\n%s\nmpack_fill_range() called with no fill function?\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->fill != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(min_bytes > 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:228\n%s\ncannot fill zero bytes!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_bytes > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(max_bytes >= min_bytes) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:230\n%s\nmin_bytes %i cannot be larger than max_bytes %i!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"max_bytes >= min_bytes\0" as *const u8 as *const ::core::ffi::c_char,
            min_bytes as ::core::ffi::c_int,
            max_bytes as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut count: size_t = 0 as size_t;
    while count < min_bytes {
        let mut read: size_t = (*reader).fill.expect("non-null function pointer")(
            reader,
            p.offset(count as isize),
            max_bytes.wrapping_sub(count),
        );
        if mpack_reader_error(reader) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 0 as size_t;
        }
        if read == 0 as size_t || read == -(1 as ::core::ffi::c_int) as size_t {
            mpack_reader_flag_error(reader, mpack_error_io);
            return 0 as size_t;
        }
        count = count.wrapping_add(read);
    }
    return count;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn mpack_reader_ensure_straddle(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> bool {
    if !(count != 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:251\n%s\ncannot ensure zero bytes!\0"
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
            b"mpack assertion failed at src/mpack/mpack-reader.c:252\n%s\nreader cannot be in an error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(count > (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:257\n%s\nstraddling ensure requested for %i bytes, but there are %i bytes left in buffer. call mpack_reader_ensure() instead\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count > (size_t)(reader->end - reader->data)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long
                as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*reader).fill.is_none() {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        return false_0 != 0;
    }
    if count > (*reader).size {
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return false_0 != 0;
    }
    let mut left: size_t =
        (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t;
    memmove(
        (*reader).buffer as *mut ::core::ffi::c_void,
        (*reader).data as *const ::core::ffi::c_void,
        left,
    );
    (*reader).end = (*reader)
        .end
        .offset(-((*reader).data.offset_from((*reader).buffer) as ::core::ffi::c_long as isize));
    (*reader).data = (*reader).buffer;
    let mut read: size_t = mpack_fill_range(
        reader,
        (*reader).buffer.offset(left as isize),
        count.wrapping_sub(left),
        (*reader).size.wrapping_sub(left),
    );
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    (*reader).end = (*reader).end.offset(read as isize);
    return true_0 != 0;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn mpack_read_native_straddle(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !p.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:295\n%s\ndata pointer for %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || p != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count,
        );
        return;
    }
    let mut left: size_t =
        (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t;
    if count <= left {
        if 0 as ::core::ffi::c_int == 0 {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-reader.c:310\n%s\nbig read requested for %i bytes, but there are %i bytes left in buffer. call mpack_read_native() instead\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                count as ::core::ffi::c_int,
                left as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        mpack_reader_flag_error(reader, mpack_error_bug);
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count,
        );
        return;
    }
    if (*reader).fill.is_none() {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count,
        );
        return;
    }
    if (*reader).size == 0 as size_t {
        mpack_reader_flag_error(reader, mpack_error_io);
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count,
        );
        return;
    }
    if left > 0 as size_t {
        memcpy(
            p as *mut ::core::ffi::c_void,
            (*reader).data as *const ::core::ffi::c_void,
            left,
        );
        count = count.wrapping_sub(left);
        p = p.offset(left as isize);
        (*reader).data = (*reader).data.offset(left as isize);
    }
    if count
        <= (*reader)
            .size
            .wrapping_div(MPACK_READER_SMALL_FRACTION_DENOMINATOR as size_t)
    {
        let mut read: size_t = mpack_fill_range(reader, (*reader).buffer, count, (*reader).size);
        if mpack_reader_error(reader) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            (*reader).buffer as *const ::core::ffi::c_void,
            count,
        );
        (*reader).data = (*reader).buffer.offset(count as isize);
        (*reader).end = (*reader).buffer.offset(read as isize);
    } else {
        mpack_fill_range(reader, p, count, count);
    };
}
#[inline(never)]
unsafe extern "C" fn mpack_skip_bytes_straddle(mut reader: *mut mpack_reader_t, mut count: size_t) {
    if (*reader).fill.is_none() {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        return;
    }
    let mut left: size_t =
        (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t;
    count = count.wrapping_sub(left);
    (*reader).data = (*reader).end;
    if (*reader).skip.is_some() && count > (*reader).size.wrapping_div(16 as size_t) {
        (*reader).skip.expect("non-null function pointer")(reader, count);
        return;
    }
    mpack_reader_skip_using_fill(reader, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_skip_bytes(mut reader: *mut mpack_reader_t, mut count: size_t) {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    mpack_reader_track_bytes(reader, count);
    let mut left: size_t =
        (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t;
    if left >= count {
        (*reader).data = (*reader).data.offset(count as isize);
        return;
    }
    mpack_skip_bytes_straddle(reader, count);
}
#[inline(never)]
unsafe extern "C" fn mpack_reader_skip_using_fill(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) {
    if (*reader).fill.is_none() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:414\n%s\nmissing fill function!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->fill != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*reader).data == (*reader).end) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:415\n%s\nthere are bytes left in the buffer!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->data == reader->end\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:416\n%s\nshould not have called this in an error state (%i)\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            (*reader).error as ::core::ffi::c_uint,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    while count > (*reader).size {
        if mpack_fill_range(reader, (*reader).buffer, (*reader).size, (*reader).size)
            < (*reader).size
        {
            mpack_reader_flag_error(reader, mpack_error_io);
            return;
        }
        count = count.wrapping_sub((*reader).size);
    }
    (*reader).data = (*reader).buffer;
    let mut read: size_t = mpack_fill_range(reader, (*reader).buffer, count, (*reader).size);
    if read < count {
        mpack_reader_flag_error(reader, mpack_error_io);
        return;
    }
    (*reader).end = (*reader).data.offset(read as isize);
    (*reader).data = (*reader).data.offset(count as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_bytes(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut count: size_t,
) {
    if p.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:442\n%s\ndestination for read of %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"p != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_reader_track_bytes(reader, count);
    mpack_read_native(reader, p, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_utf8(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut byte_count: size_t,
) {
    if p.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:448\n%s\ndestination for read of %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"p != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            byte_count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_reader_track_str_bytes_all(reader, byte_count);
    mpack_read_native(reader, p, byte_count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && !mpack_utf8_check(p, byte_count)
    {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
unsafe extern "C" fn mpack_read_cstr_unchecked(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut byte_count: size_t,
) {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:457\n%s\ndestination for read of %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            byte_count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(buffer_size >= 1 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:458\n%s\nbuffer size is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer_size >= 1\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_reader_error(reader) as u64 != 0 {
        *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        return;
    }
    if byte_count > buffer_size.wrapping_sub(1 as size_t) {
        mpack_reader_flag_error(reader, mpack_error_too_big);
        *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        return;
    }
    mpack_reader_track_str_bytes_all(reader, byte_count);
    mpack_read_native(reader, buf, byte_count);
    *buf.offset(byte_count as isize) = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_cstr(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut byte_count: size_t,
) {
    mpack_read_cstr_unchecked(reader, buf, buffer_size, byte_count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && !mpack_str_check_no_null(buf, byte_count)
    {
        *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_utf8_cstr(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut byte_count: size_t,
) {
    mpack_read_cstr_unchecked(reader, buf, buffer_size, byte_count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && !mpack_utf8_check_no_null(buf, byte_count)
    {
        *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
unsafe extern "C" fn mpack_read_native_noerrorfn(
    mut reader: *mut mpack_reader_t,
    mut p: *mut ::core::ffi::c_char,
    mut count: size_t,
) {
    if !((*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:501\n%s\ncannot call if an error is already flagged!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut error_fn: mpack_reader_error_t = (*reader).error_fn;
    (*reader).error_fn = None;
    mpack_read_native(reader, p, count);
    (*reader).error_fn = error_fn;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_bytes_alloc_impl(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
    mut null_terminated: bool,
) -> *mut ::core::ffi::c_char {
    mpack_reader_track_bytes(reader, count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if count == 0 as size_t && null_terminated as ::core::ffi::c_int == false_0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut data: *mut ::core::ffi::c_char = test_malloc(count.wrapping_add(
        (if null_terminated as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t,
    )) as *mut ::core::ffi::c_char;
    if data.is_null() {
        mpack_reader_flag_error(reader, mpack_error_memory);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    mpack_read_native_noerrorfn(reader, data, count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        test_free(data as *mut ::core::ffi::c_void);
        if (*reader).error_fn.is_some() {
            (*reader).error_fn.expect("non-null function pointer")(
                reader,
                mpack_reader_error(reader),
            );
        }
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if null_terminated {
        *data.offset(count as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return data;
}
unsafe extern "C" fn mpack_read_bytes_inplace_notrack(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> *const ::core::ffi::c_char {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if (*reader).end.offset_from((*reader).data) as ::core::ffi::c_long as size_t >= count {
        let mut bytes: *const ::core::ffi::c_char = (*reader).data;
        (*reader).data = (*reader).data.offset(count as isize);
        return bytes;
    }
    if !mpack_reader_ensure(reader, count) {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut bytes_0: *const ::core::ffi::c_char = (*reader).data;
    (*reader).data = (*reader).data.offset(count as isize);
    return bytes_0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_bytes_inplace(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> *const ::core::ffi::c_char {
    mpack_reader_track_bytes(reader, count);
    return mpack_read_bytes_inplace_notrack(reader, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_utf8_inplace(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> *const ::core::ffi::c_char {
    mpack_reader_track_str_bytes_all(reader, count);
    let mut str: *const ::core::ffi::c_char = mpack_read_bytes_inplace_notrack(reader, count);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && !mpack_utf8_check(str, count)
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return str;
}
unsafe extern "C" fn mpack_parse_tag(
    mut reader: *mut mpack_reader_t,
    mut tag: *mut mpack_tag_t,
) -> size_t {
    if !((*reader).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:582\n%s\nreader cannot be in an error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"reader->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !mpack_reader_ensure(reader, 1 as size_t) {
        return 0 as size_t;
    }
    let mut type_0: uint8_t = mpack_load_u8((*reader).data);
    match type_0 as ::core::ffi::c_int {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
        | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36
        | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53
        | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70
        | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87
        | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103
        | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117
        | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 => {
            *tag = mpack_tag_make_uint(type_0 as uint64_t);
            return 1 as size_t;
        }
        224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237 | 238
        | 239 | 240 | 241 | 242 | 243 | 244 | 245 | 246 | 247 | 248 | 249 | 250 | 251 | 252
        | 253 | 254 | 255 => {
            *tag = mpack_tag_make_int(type_0 as int8_t as int64_t);
            return 1 as size_t;
        }
        128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142
        | 143 => {
            *tag = mpack_tag_make_map(type_0 as uint32_t & !(0xf0 as uint32_t));
            return 1 as size_t;
        }
        144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158
        | 159 => {
            *tag = mpack_tag_make_array(type_0 as uint32_t & !(0xf0 as uint32_t));
            return 1 as size_t;
        }
        160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174
        | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188
        | 189 | 190 | 191 => {
            *tag = mpack_tag_make_str(type_0 as uint32_t & !(0xe0 as uint32_t));
            return 1 as size_t;
        }
        192 => {
            *tag = mpack_tag_make_nil();
            return 1 as size_t;
        }
        194 | 195 => {
            *tag = mpack_tag_make_bool(type_0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0);
            return 1 as size_t;
        }
        196 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_BIN8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_bin(mpack_load_u8(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_BIN8 as size_t;
        }
        197 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_BIN16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_bin(mpack_load_u16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_BIN16 as size_t;
        }
        198 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_BIN32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_bin(mpack_load_u32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_BIN32 as size_t;
        }
        199 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_EXT8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(2 as ::core::ffi::c_int as isize)),
                mpack_load_u8((*reader).data.offset(1 as ::core::ffi::c_int as isize)) as uint32_t,
            );
            return MPACK_TAG_SIZE_EXT8 as size_t;
        }
        200 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_EXT16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(3 as ::core::ffi::c_int as isize)),
                mpack_load_u16((*reader).data.offset(1 as ::core::ffi::c_int as isize)) as uint32_t,
            );
            return MPACK_TAG_SIZE_EXT16 as size_t;
        }
        201 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_EXT32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(5 as ::core::ffi::c_int as isize)),
                mpack_load_u32((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
            );
            return MPACK_TAG_SIZE_EXT32 as size_t;
        }
        202 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FLOAT as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_float(mpack_load_float(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_FLOAT as size_t;
        }
        203 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_DOUBLE as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_double(mpack_load_double(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_DOUBLE as size_t;
        }
        204 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_U8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_uint(mpack_load_u8(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t);
            return MPACK_TAG_SIZE_U8 as size_t;
        }
        205 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_U16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_uint(mpack_load_u16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t);
            return MPACK_TAG_SIZE_U16 as size_t;
        }
        206 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_U32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_uint(mpack_load_u32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint64_t);
            return MPACK_TAG_SIZE_U32 as size_t;
        }
        207 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_U64 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_uint(mpack_load_u64(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_U64 as size_t;
        }
        208 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_I8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_int(mpack_load_i8(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t);
            return MPACK_TAG_SIZE_I8 as size_t;
        }
        209 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_I16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_int(mpack_load_i16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t);
            return MPACK_TAG_SIZE_I16 as size_t;
        }
        210 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_I32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_int(mpack_load_i32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as int64_t);
            return MPACK_TAG_SIZE_I32 as size_t;
        }
        211 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_I64 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_int(mpack_load_i64(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_I64 as size_t;
        }
        212 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FIXEXT1 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
                1 as uint32_t,
            );
            return MPACK_TAG_SIZE_FIXEXT1 as size_t;
        }
        213 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FIXEXT2 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
                2 as uint32_t,
            );
            return MPACK_TAG_SIZE_FIXEXT2 as size_t;
        }
        214 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FIXEXT4 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
                4 as uint32_t,
            );
            return 2 as size_t;
        }
        215 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FIXEXT8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
                8 as uint32_t,
            );
            return MPACK_TAG_SIZE_FIXEXT8 as size_t;
        }
        216 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_FIXEXT16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_ext(
                mpack_load_i8((*reader).data.offset(1 as ::core::ffi::c_int as isize)),
                16 as uint32_t,
            );
            return MPACK_TAG_SIZE_FIXEXT16 as size_t;
        }
        217 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_STR8 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_str(mpack_load_u8(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_STR8 as size_t;
        }
        218 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_STR16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_str(mpack_load_u16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_STR16 as size_t;
        }
        219 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_STR32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_str(mpack_load_u32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_STR32 as size_t;
        }
        220 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_ARRAY16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_array(mpack_load_u16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_ARRAY16 as size_t;
        }
        221 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_ARRAY32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_array(mpack_load_u32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_ARRAY32 as size_t;
        }
        222 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_MAP16 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_map(mpack_load_u16(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ) as uint32_t);
            return MPACK_TAG_SIZE_MAP16 as size_t;
        }
        223 => {
            if !mpack_reader_ensure(reader, MPACK_TAG_SIZE_MAP32 as size_t) {
                return 0 as size_t;
            }
            *tag = mpack_tag_make_map(mpack_load_u32(
                (*reader).data.offset(1 as ::core::ffi::c_int as isize),
            ));
            return MPACK_TAG_SIZE_MAP32 as size_t;
        }
        193 => {
            mpack_reader_flag_error(reader, mpack_error_invalid);
            return 0 as size_t;
        }
        _ => {}
    }
    if 0 as ::core::ffi::c_int == 0 {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:929\n%s\nunreachable\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_tag(mut reader: *mut mpack_reader_t) -> mpack_tag_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_tag_nil();
    }
    if mpack_reader_track_element(reader) as ::core::ffi::c_uint
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
    let mut count: size_t = mpack_parse_tag(reader, &raw mut tag);
    if count == 0 as size_t {
        return mpack_tag_nil();
    }
    let mut track_error: mpack_error_t = mpack_ok;
    match tag.type_0 as ::core::ffi::c_uint {
        10 | 9 => {
            track_error = mpack_track_push(&raw mut (*reader).track, tag.type_0, tag.v.n);
        }
        11 | 7 | 8 => {
            track_error = mpack_track_push(&raw mut (*reader).track, tag.type_0, tag.v.l);
        }
        _ => {}
    }
    if track_error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        mpack_reader_flag_error(reader, track_error);
        return mpack_tag_nil();
    }
    (*reader).data = (*reader).data.offset(count as isize);
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_peek_tag(mut reader: *mut mpack_reader_t) -> mpack_tag_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return mpack_tag_nil();
    }
    if mpack_reader_track_peek_element(reader) as ::core::ffi::c_uint
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
    if mpack_parse_tag(reader, &raw mut tag) == 0 as size_t {
        return mpack_tag_nil();
    }
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_discard(mut reader: *mut mpack_reader_t) {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if mpack_reader_error(reader) as u64 != 0 {
        return;
    }
    match var.type_0 as ::core::ffi::c_uint {
        7 => {
            mpack_skip_bytes(reader, var.v.l as size_t);
            mpack_done_str(reader);
        }
        8 => {
            mpack_skip_bytes(reader, var.v.l as size_t);
            mpack_done_bin(reader);
        }
        11 => {
            mpack_skip_bytes(reader, var.v.l as size_t);
            mpack_done_ext(reader);
        }
        9 => {
            while var.v.n > 0 as uint32_t {
                mpack_discard(reader);
                if mpack_reader_error(reader) as u64 != 0 {
                    break;
                }
                var.v.n = var.v.n.wrapping_sub(1);
            }
            mpack_done_array(reader);
        }
        10 => {
            while var.v.n > 0 as uint32_t {
                mpack_discard(reader);
                mpack_discard(reader);
                if mpack_reader_error(reader) as u64 != 0 {
                    break;
                }
                var.v.n = var.v.n.wrapping_sub(1);
            }
            mpack_done_map(reader);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_read_timestamp(
    mut reader: *mut mpack_reader_t,
    mut size: size_t,
) -> mpack_timestamp_t {
    let mut timestamp: mpack_timestamp_t = mpack_timestamp_t {
        seconds: 0 as int64_t,
        nanoseconds: 0 as uint32_t,
    };
    if size != 4 as size_t && size != 8 as size_t && size != 12 as size_t {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        return timestamp;
    }
    let mut buf: [::core::ffi::c_char; 12] = [0; 12];
    mpack_read_bytes(reader, &raw mut buf as *mut ::core::ffi::c_char, size);
    mpack_done_ext(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return timestamp;
    }
    match size {
        4 => {
            timestamp.seconds =
                mpack_load_u32(&raw mut buf as *mut ::core::ffi::c_char) as uint64_t as int64_t;
        }
        8 => {
            let mut packed: uint64_t = mpack_load_u64(&raw mut buf as *mut ::core::ffi::c_char);
            timestamp.seconds = (packed
                & ((1 as uint64_t) << 34 as ::core::ffi::c_int).wrapping_sub(1 as uint64_t))
                as int64_t;
            timestamp.nanoseconds = (packed >> 34 as ::core::ffi::c_int) as uint32_t;
        }
        12 => {
            timestamp.nanoseconds = mpack_load_u32(&raw mut buf as *mut ::core::ffi::c_char);
            timestamp.seconds = mpack_load_i64(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(4 as ::core::ffi::c_int as isize),
            );
        }
        _ => {
            if 0 as ::core::ffi::c_int == 0 {
                mpack_assert_fail_format(
                    b"mpack assertion failed at src/mpack/mpack-reader.c:1067\n%s\nunreachable\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            } else {
            };
        }
    }
    if timestamp.nanoseconds > MPACK_TIMESTAMP_NANOSECONDS_MAX as uint32_t {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        let mut zero: mpack_timestamp_t = mpack_timestamp_t {
            seconds: 0 as int64_t,
            nanoseconds: 0 as uint32_t,
        };
        return zero;
    }
    return timestamp;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_done_type(
    mut reader: *mut mpack_reader_t,
    mut type_0: mpack_type_t,
) {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_if_error(reader, mpack_track_pop(&raw mut (*reader).track, type_0));
    }
}
unsafe extern "C" fn mpack_print_read_prefix(
    mut reader: *mut mpack_reader_t,
    mut length: size_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> size_t {
    if length == 0 as size_t {
        return 0 as size_t;
    }
    let mut read: size_t = if length < buffer_size {
        length
    } else {
        buffer_size
    };
    mpack_read_bytes(reader, buffer, read);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as size_t;
    }
    mpack_skip_bytes(reader, length.wrapping_sub(read));
    return read;
}
unsafe extern "C" fn mpack_print_element(
    mut reader: *mut mpack_reader_t,
    mut print: *mut mpack_print_t,
    mut depth: size_t,
) {
    let mut val: mpack_tag_t = mpack_read_tag(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut buffer: [::core::ffi::c_char; 12] = [0; 12];
    let mut count: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match val.type_0 as ::core::ffi::c_uint {
        7 => {
            mpack_print_append_cstr(print, b"\"\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as size_t;
            while i < val.v.l as size_t {
                let mut c: ::core::ffi::c_char = 0;
                mpack_read_bytes(reader, &raw mut c, 1 as size_t);
                if mpack_reader_error(reader) as ::core::ffi::c_uint
                    != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return;
                }
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
            mpack_done_str(reader);
            return;
        }
        9 => {
            mpack_print_append_cstr(print, b"[\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as size_t;
            while i < val.v.n as size_t {
                j = 0 as size_t;
                while j < depth.wrapping_add(1 as size_t) {
                    mpack_print_append_cstr(
                        print,
                        b"    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    j = j.wrapping_add(1);
                }
                mpack_print_element(reader, print, depth.wrapping_add(1 as size_t));
                if mpack_reader_error(reader) as ::core::ffi::c_uint
                    != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return;
                }
                if i != val.v.n.wrapping_sub(1 as uint32_t) as size_t {
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
            mpack_done_array(reader);
            return;
        }
        10 => {
            mpack_print_append_cstr(print, b"{\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as size_t;
            while i < val.v.n as size_t {
                j = 0 as size_t;
                while j < depth.wrapping_add(1 as size_t) {
                    mpack_print_append_cstr(
                        print,
                        b"    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    j = j.wrapping_add(1);
                }
                mpack_print_element(reader, print, depth.wrapping_add(1 as size_t));
                if mpack_reader_error(reader) as ::core::ffi::c_uint
                    != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return;
                }
                mpack_print_append_cstr(print, b": \0" as *const u8 as *const ::core::ffi::c_char);
                mpack_print_element(reader, print, depth.wrapping_add(1 as size_t));
                if mpack_reader_error(reader) as ::core::ffi::c_uint
                    != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return;
                }
                if i != val.v.n.wrapping_sub(1 as uint32_t) as size_t {
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
            mpack_done_map(reader);
            return;
        }
        8 => {
            count = mpack_print_read_prefix(
                reader,
                mpack_tag_bin_length(&raw mut val) as size_t,
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
            );
            mpack_done_bin(reader);
        }
        11 => {
            count = mpack_print_read_prefix(
                reader,
                mpack_tag_ext_length(&raw mut val) as size_t,
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
            );
            mpack_done_ext(reader);
        }
        _ => {}
    }
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    mpack_tag_debug_pseudo_json(
        val,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        &raw mut buffer as *mut ::core::ffi::c_char,
        count,
    );
    mpack_print_append_cstr(print, &raw mut buf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn mpack_print_and_destroy(
    mut reader: *mut mpack_reader_t,
    mut print: *mut mpack_print_t,
    mut depth: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < depth {
        mpack_print_append_cstr(print, b"    \0" as *const u8 as *const ::core::ffi::c_char);
        i = i.wrapping_add(1);
    }
    mpack_print_element(reader, print, depth);
    let mut remaining: size_t = mpack_reader_remaining(
        reader,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    if mpack_reader_destroy(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            b"\n<mpack parsing error %s>\0" as *const u8 as *const ::core::ffi::c_char,
            mpack_error_to_string(mpack_reader_error(reader)),
        );
        buf[(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize).wrapping_sub(1 as usize)
            as usize] = '\0' as i32 as ::core::ffi::c_char;
        mpack_print_append_cstr(print, &raw mut buf as *mut ::core::ffi::c_char);
    } else if remaining > 0 as size_t {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            b"\n<%i extra bytes at end of message>\0" as *const u8 as *const ::core::ffi::c_char,
            remaining as ::core::ffi::c_int,
        );
        buf[(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize).wrapping_sub(1 as usize)
            as usize] = '\0' as i32 as ::core::ffi::c_char;
        mpack_print_append_cstr(print, &raw mut buf as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn mpack_print_data(
    mut data: *const ::core::ffi::c_char,
    mut len: size_t,
    mut print: *mut mpack_print_t,
    mut depth: size_t,
) {
    let mut reader: mpack_reader_t = mpack_reader_t {
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        fill: None,
        error_fn: None,
        teardown: None,
        skip: None,
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        data: ::core::ptr::null::<::core::ffi::c_char>(),
        end: ::core::ptr::null::<::core::ffi::c_char>(),
        error: mpack_ok,
        track: mpack_track_t {
            count: 0,
            capacity: 0,
            elements: ::core::ptr::null_mut::<mpack_track_element_t>(),
        },
    };
    mpack_reader_init_data(&raw mut reader, data, len);
    mpack_print_and_destroy(&raw mut reader, print, depth);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_data_to_buffer(
    mut data: *const ::core::ffi::c_char,
    mut data_size: size_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) {
    if buffer_size == 0 as size_t {
        if 0 as ::core::ffi::c_int == 0 {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-reader.c:1223\n%s\nbuffer size is zero!\0"
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
    mpack_print_data(data, data_size, &raw mut print, 0 as size_t);
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
pub unsafe extern "C" fn mpack_print_data_to_callback(
    mut data: *const ::core::ffi::c_char,
    mut size: size_t,
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
    mpack_print_data(data, size, &raw mut print, 0 as size_t);
    mpack_print_flush(&raw mut print);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_data_to_file(
    mut data: *const ::core::ffi::c_char,
    mut len: size_t,
    mut file: *mut FILE,
) {
    if data.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:1253\n%s\ndata is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"data != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if file.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-reader.c:1254\n%s\nfile is NULL\0"
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
    mpack_print_data(data, len, &raw mut print, 2 as size_t);
    mpack_print_append_cstr(
        &raw mut print,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    mpack_print_flush(&raw mut print);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_stdfile_to_callback(
    mut file: *mut FILE,
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
    let mut reader: mpack_reader_t = mpack_reader_t {
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        fill: None,
        error_fn: None,
        teardown: None,
        skip: None,
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        data: ::core::ptr::null::<::core::ffi::c_char>(),
        end: ::core::ptr::null::<::core::ffi::c_char>(),
        error: mpack_ok,
        track: mpack_track_t {
            count: 0,
            capacity: 0,
            elements: ::core::ptr::null_mut::<mpack_track_element_t>(),
        },
    };
    mpack_reader_init_stdfile(&raw mut reader, file, false_0 != 0);
    mpack_print_and_destroy(&raw mut reader, &raw mut print, 0 as size_t);
    mpack_print_flush(&raw mut print);
}
pub const MPACK_BUFFER_SIZE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
