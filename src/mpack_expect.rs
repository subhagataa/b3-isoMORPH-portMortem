#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn test_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn test_free(p: *mut ::core::ffi::c_void);
    fn test_strlen(s: *const ::core::ffi::c_char) -> size_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn mpack_assert_fail_format(format: *const ::core::ffi::c_char, ...) -> !;
    fn mpack_break_hit_format(format: *const ::core::ffi::c_char, ...);
    fn mpack_tag_cmp(left: mpack_tag_t, right: mpack_tag_t) -> ::core::ffi::c_int;
    fn mpack_track_push(
        track: *mut mpack_track_t,
        type_0: mpack_type_t,
        count: uint32_t,
    ) -> mpack_error_t;
    fn mpack_track_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_bytes(track: *mut mpack_track_t, read: bool, count: size_t) -> mpack_error_t;
    fn mpack_utf8_check(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
    fn mpack_utf8_check_no_null(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
    fn mpack_str_check_no_null(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
    fn mpack_reader_flag_error(reader: *mut mpack_reader_t, error: mpack_error_t);
    fn mpack_read_tag(reader: *mut mpack_reader_t) -> mpack_tag_t;
    fn mpack_peek_tag(reader: *mut mpack_reader_t) -> mpack_tag_t;
    fn mpack_read_bytes(reader: *mut mpack_reader_t, p: *mut ::core::ffi::c_char, count: size_t);
    fn mpack_read_cstr(
        reader: *mut mpack_reader_t,
        buf: *mut ::core::ffi::c_char,
        buffer_size: size_t,
        byte_count: size_t,
    );
    fn mpack_read_utf8_cstr(
        reader: *mut mpack_reader_t,
        buf: *mut ::core::ffi::c_char,
        buffer_size: size_t,
        byte_count: size_t,
    );
    fn mpack_read_bytes_alloc_impl(
        reader: *mut mpack_reader_t,
        count: size_t,
        null_terminated: bool,
    ) -> *mut ::core::ffi::c_char;
    fn mpack_read_bytes_inplace(
        reader: *mut mpack_reader_t,
        count: size_t,
    ) -> *const ::core::ffi::c_char;
    fn mpack_read_timestamp(reader: *mut mpack_reader_t, size: size_t) -> mpack_timestamp_t;
    fn mpack_done_type(reader: *mut mpack_reader_t, type_0: mpack_type_t);
    fn mpack_discard(reader: *mut mpack_reader_t);
    fn mpack_reader_ensure_straddle(reader: *mut mpack_reader_t, count: size_t) -> bool;
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
pub const MPACK_INT8_MAX: ::core::ffi::c_int = INT8_MAX;
pub const MPACK_INT16_MAX: ::core::ffi::c_int = INT16_MAX;
pub const MPACK_INT32_MAX: ::core::ffi::c_int = INT32_MAX;
pub const MPACK_INT64_MAX: ::core::ffi::c_long = INT64_MAX;
pub const MPACK_UINT8_MAX: ::core::ffi::c_int = UINT8_MAX;
pub const MPACK_UINT16_MAX: ::core::ffi::c_int = UINT16_MAX;
pub const MPACK_UINT32_MAX: ::core::ffi::c_uint = UINT32_MAX;
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
unsafe extern "C" fn mpack_tag_ext_exttype(mut tag: *mut mpack_tag_t) -> int8_t {
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
pub const MPACK_EXTTYPE_TIMESTAMP: int8_t = -(1 as ::core::ffi::c_int) as int8_t;
#[inline]
unsafe extern "C" fn mpack_tag_equal(mut left: mpack_tag_t, mut right: mpack_tag_t) -> bool {
    return mpack_tag_cmp(left, right) == 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn mpack_read_bytes_alloc(
    mut reader: *mut mpack_reader_t,
    mut count: size_t,
) -> *mut ::core::ffi::c_char {
    return mpack_read_bytes_alloc_impl(reader, count, false_0 != 0);
}
#[inline]
unsafe extern "C" fn mpack_done_array(mut reader: *mut mpack_reader_t) {
    mpack_done_type(reader, mpack_type_array);
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
unsafe extern "C" fn mpack_expect_array_max(
    mut reader: *mut mpack_reader_t,
    mut max_count: uint32_t,
) -> uint32_t {
    return mpack_expect_array_range(reader, 0 as uint32_t, max_count);
}
#[inline]
unsafe extern "C" fn mpack_expect_str_max(
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
#[inline]
unsafe extern "C" fn mpack_expect_str_length(mut reader: *mut mpack_reader_t, mut count: uint32_t) {
    if mpack_expect_str(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[inline]
unsafe extern "C" fn mpack_expect_bin_max(
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
#[inline]
unsafe extern "C" fn mpack_expect_bin_size(mut reader: *mut mpack_reader_t, mut count: uint32_t) {
    if mpack_expect_bin(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[inline]
unsafe extern "C" fn mpack_expect_ext_max(
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
#[inline]
unsafe extern "C" fn mpack_expect_native_u8(mut reader: *mut mpack_reader_t) -> uint8_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint8_t;
    }
    let mut type_0: uint8_t = 0;
    if !mpack_reader_ensure(reader, ::core::mem::size_of::<uint8_t>() as size_t) {
        return 0 as uint8_t;
    }
    type_0 = mpack_load_u8((*reader).data);
    (*reader).data = (*reader)
        .data
        .offset(::core::mem::size_of::<uint8_t>() as usize as isize);
    return type_0;
}
#[inline]
unsafe extern "C" fn mpack_expect_native_u16(mut reader: *mut mpack_reader_t) -> uint16_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint16_t;
    }
    let mut type_0: uint16_t = 0;
    if !mpack_reader_ensure(reader, ::core::mem::size_of::<uint16_t>() as size_t) {
        return 0 as uint16_t;
    }
    type_0 = mpack_load_u16((*reader).data);
    (*reader).data = (*reader)
        .data
        .offset(::core::mem::size_of::<uint16_t>() as usize as isize);
    return type_0;
}
#[inline]
unsafe extern "C" fn mpack_expect_native_u32(mut reader: *mut mpack_reader_t) -> uint32_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    let mut type_0: uint32_t = 0;
    if !mpack_reader_ensure(reader, ::core::mem::size_of::<uint32_t>() as size_t) {
        return 0 as uint32_t;
    }
    type_0 = mpack_load_u32((*reader).data);
    (*reader).data = (*reader)
        .data
        .offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    return type_0;
}
#[inline]
unsafe extern "C" fn mpack_expect_type_byte(mut reader: *mut mpack_reader_t) -> uint8_t {
    mpack_reader_track_element(reader);
    return mpack_expect_native_u8(reader);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u8(mut reader: *mut mpack_reader_t) -> uint8_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_UINT8_MAX as uint64_t {
            return var.v.u as uint8_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= 0 as int64_t && var.v.i <= MPACK_UINT8_MAX as int64_t {
            return var.v.i as uint8_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u16(mut reader: *mut mpack_reader_t) -> uint16_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_UINT16_MAX as uint64_t {
            return var.v.u as uint16_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= 0 as int64_t && var.v.i <= MPACK_UINT16_MAX as int64_t {
            return var.v.i as uint16_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u32(mut reader: *mut mpack_reader_t) -> uint32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_UINT32_MAX as uint64_t {
            return var.v.u as uint32_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= 0 as int64_t && var.v.i <= MPACK_UINT32_MAX as int64_t {
            return var.v.i as uint32_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u64(mut reader: *mut mpack_reader_t) -> uint64_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.u;
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= 0 as int64_t {
            return var.v.i as uint64_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i8(mut reader: *mut mpack_reader_t) -> int8_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_INT8_MAX as uint64_t {
            return var.v.u as int8_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= MPACK_INT8_MIN as int64_t && var.v.i <= MPACK_INT8_MAX as int64_t {
            return var.v.i as int8_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i16(mut reader: *mut mpack_reader_t) -> int16_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_INT16_MAX as uint64_t {
            return var.v.u as int16_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= MPACK_INT16_MIN as int64_t && var.v.i <= MPACK_INT16_MAX as int64_t {
            return var.v.i as int16_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i32(mut reader: *mut mpack_reader_t) -> int32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_INT32_MAX as uint64_t {
            return var.v.u as int32_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.i >= MPACK_INT32_MIN as int64_t && var.v.i <= MPACK_INT32_MAX as int64_t {
            return var.v.i as int32_t;
        }
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i64(mut reader: *mut mpack_reader_t) -> int64_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if var.v.u <= MPACK_INT64_MAX as uint64_t {
            return var.v.u as int64_t;
        }
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.i;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_float(
    mut reader: *mut mpack_reader_t,
) -> ::core::ffi::c_float {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.u as ::core::ffi::c_float;
    }
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.i as ::core::ffi::c_float;
    }
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.f;
    }
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.d as ::core::ffi::c_float;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_double(
    mut reader: *mut mpack_reader_t,
) -> ::core::ffi::c_double {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.u as ::core::ffi::c_double;
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.i as ::core::ffi::c_double;
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.f as ::core::ffi::c_double;
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.d;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_float_strict(
    mut reader: *mut mpack_reader_t,
) -> ::core::ffi::c_float {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.f;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_double_strict(
    mut reader: *mut mpack_reader_t,
) -> ::core::ffi::c_double {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_float as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.f as ::core::ffi::c_double;
    } else if var.type_0 as ::core::ffi::c_uint
        == mpack_type_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.d;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u8_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint8_t,
    mut max_value: uint8_t,
) -> uint8_t {
    if !(min_value as ::core::ffi::c_int <= max_value as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:286\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value as ::core::ffi::c_int,
            max_value as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint8_t = mpack_expect_u8(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if (val as ::core::ffi::c_int) < min_value as ::core::ffi::c_int
        || val as ::core::ffi::c_int > max_value as ::core::ffi::c_int
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u16_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint16_t,
    mut max_value: uint16_t,
) -> uint16_t {
    if !(min_value as ::core::ffi::c_int <= max_value as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:287\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value as ::core::ffi::c_int,
            max_value as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint16_t = mpack_expect_u16(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if (val as ::core::ffi::c_int) < min_value as ::core::ffi::c_int
        || val as ::core::ffi::c_int > max_value as ::core::ffi::c_int
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u32_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint32_t,
    mut max_value: uint32_t,
) -> uint32_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:288\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint32_t = mpack_expect_u32(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_u64_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint64_t,
    mut max_value: uint64_t,
) -> uint64_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:289\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint64_t = mpack_expect_u64(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i8_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: int8_t,
    mut max_value: int8_t,
) -> int8_t {
    if !(min_value as ::core::ffi::c_int <= max_value as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:291\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value as ::core::ffi::c_int,
            max_value as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: int8_t = mpack_expect_i8(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if (val as ::core::ffi::c_int) < min_value as ::core::ffi::c_int
        || val as ::core::ffi::c_int > max_value as ::core::ffi::c_int
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i16_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: int16_t,
    mut max_value: int16_t,
) -> int16_t {
    if !(min_value as ::core::ffi::c_int <= max_value as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:292\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value as ::core::ffi::c_int,
            max_value as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: int16_t = mpack_expect_i16(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if (val as ::core::ffi::c_int) < min_value as ::core::ffi::c_int
        || val as ::core::ffi::c_int > max_value as ::core::ffi::c_int
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i32_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: int32_t,
    mut max_value: int32_t,
) -> int32_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:293\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: int32_t = mpack_expect_i32(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_i64_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: int64_t,
    mut max_value: int64_t,
) -> int64_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:294\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: int64_t = mpack_expect_i64(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_float_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: ::core::ffi::c_float,
    mut max_value: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:297\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value as ::core::ffi::c_double,
            max_value as ::core::ffi::c_double,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: ::core::ffi::c_float = mpack_expect_float(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_double_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: ::core::ffi::c_double,
    mut max_value: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:300\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: ::core::ffi::c_double = mpack_expect_double(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_map_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint32_t,
    mut max_value: uint32_t,
) -> uint32_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:303\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint32_t = mpack_expect_map(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array_range(
    mut reader: *mut mpack_reader_t,
    mut min_value: uint32_t,
    mut max_value: uint32_t,
) -> uint32_t {
    if !(min_value <= max_value) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:304\n%s\nmin_value %i must be less than or equal to max_value %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"min_value <= max_value\0" as *const u8 as *const ::core::ffi::c_char,
            min_value,
            max_value,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut val: uint32_t = mpack_expect_array(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return min_value;
    }
    if val < min_value || val > max_value {
        mpack_reader_flag_error(reader, mpack_error_type);
        return min_value;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_uint_match(
    mut reader: *mut mpack_reader_t,
    mut value: uint64_t,
) {
    if mpack_expect_u64(reader) != value {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_int_match(
    mut reader: *mut mpack_reader_t,
    mut value: int64_t,
) {
    if mpack_expect_i64(reader) != value {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_nil(mut reader: *mut mpack_reader_t) {
    if mpack_expect_type_byte(reader) as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_bool(mut reader: *mut mpack_reader_t) -> bool {
    let mut type_0: uint8_t = mpack_expect_type_byte(reader);
    if type_0 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int) != 0xc2 as ::core::ffi::c_int {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
    return type_0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_true(mut reader: *mut mpack_reader_t) {
    if mpack_expect_bool(reader) as ::core::ffi::c_int != true_0 {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_false(mut reader: *mut mpack_reader_t) {
    if mpack_expect_bool(reader) as ::core::ffi::c_int != false_0 {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_timestamp(
    mut reader: *mut mpack_reader_t,
) -> mpack_timestamp_t {
    let mut zero: mpack_timestamp_t = mpack_timestamp_t {
        seconds: 0 as int64_t,
        nanoseconds: 0 as uint32_t,
    };
    let mut tag: mpack_tag_t = mpack_read_tag(reader);
    if tag.type_0 as ::core::ffi::c_uint
        != mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return zero;
    }
    if mpack_tag_ext_exttype(&raw mut tag) as ::core::ffi::c_int
        != MPACK_EXTTYPE_TIMESTAMP as ::core::ffi::c_int
    {
        mpack_reader_flag_error(reader, mpack_error_type);
        return zero;
    }
    return mpack_read_timestamp(reader, mpack_tag_ext_length(&raw mut tag) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_timestamp_truncate(
    mut reader: *mut mpack_reader_t,
) -> int64_t {
    return mpack_expect_timestamp(reader).seconds;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_map(mut reader: *mut mpack_reader_t) -> uint32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.n;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_map_match(
    mut reader: *mut mpack_reader_t,
    mut count: uint32_t,
) {
    if mpack_expect_map(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_map_or_nil(
    mut reader: *mut mpack_reader_t,
    mut count: *mut uint32_t,
) -> bool {
    if count.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:383\n%s\ncount cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_nil as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *count = 0 as uint32_t;
        return false_0 != 0;
    }
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *count = var.v.n;
        return true_0 != 0;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    *count = 0 as uint32_t;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_map_max_or_nil(
    mut reader: *mut mpack_reader_t,
    mut max_count: uint32_t,
    mut count: *mut uint32_t,
) -> bool {
    if count.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:400\n%s\ncount cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut has_map: bool = mpack_expect_map_or_nil(reader, count);
    if has_map as ::core::ffi::c_int != 0 && *count > max_count {
        *count = 0 as uint32_t;
        mpack_reader_flag_error(reader, mpack_error_type);
        return false_0 != 0;
    }
    return has_map;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array(mut reader: *mut mpack_reader_t) -> uint32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.n;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array_match(
    mut reader: *mut mpack_reader_t,
    mut count: uint32_t,
) {
    if mpack_expect_array(reader) != count {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array_or_nil(
    mut reader: *mut mpack_reader_t,
    mut count: *mut uint32_t,
) -> bool {
    if count.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:425\n%s\ncount cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_nil as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *count = 0 as uint32_t;
        return false_0 != 0;
    }
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *count = var.v.n;
        return true_0 != 0;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    *count = 0 as uint32_t;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array_max_or_nil(
    mut reader: *mut mpack_reader_t,
    mut max_count: uint32_t,
    mut count: *mut uint32_t,
) -> bool {
    if count.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:442\n%s\ncount cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut has_array: bool = mpack_expect_array_or_nil(reader, count);
    if has_array as ::core::ffi::c_int != 0 && *count > max_count {
        *count = 0 as uint32_t;
        mpack_reader_flag_error(reader, mpack_error_type);
        return false_0 != 0;
    }
    return has_array;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_array_alloc_impl(
    mut reader: *mut mpack_reader_t,
    mut element_size: size_t,
    mut max_count: uint32_t,
    mut out_count: *mut uint32_t,
    mut allow_nil: bool,
) -> *mut ::core::ffi::c_void {
    if out_count.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:455\n%s\nout_count cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"out_count != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *out_count = 0 as uint32_t;
    let mut count: uint32_t = 0;
    let mut has_array: bool = true_0 != 0;
    if allow_nil {
        has_array = mpack_expect_array_max_or_nil(reader, max_count, &raw mut count);
    } else {
        count = mpack_expect_array_max(reader, max_count);
    }
    if mpack_reader_error(reader) as u64 != 0 {
        return NULL;
    }
    if count == 0 as uint32_t {
        if allow_nil as ::core::ffi::c_int != 0 && has_array as ::core::ffi::c_int != 0 {
            mpack_done_array(reader);
        }
        return NULL;
    }
    let mut p: *mut ::core::ffi::c_void = test_malloc(element_size.wrapping_mul(count as size_t));
    if p.is_null() {
        mpack_reader_flag_error(reader, mpack_error_memory);
        return NULL;
    }
    *out_count = count;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_str(mut reader: *mut mpack_reader_t) -> uint32_t {
    let mut type_0: uint8_t = mpack_expect_type_byte(reader);
    let mut count: uint32_t = 0;
    if type_0 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
        count = (type_0 as ::core::ffi::c_int
            & !(0xe0 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int)
            as uint32_t;
    } else if type_0 as ::core::ffi::c_int == 0xd9 as ::core::ffi::c_int {
        count = mpack_expect_native_u8(reader) as uint32_t;
    } else if type_0 as ::core::ffi::c_int == 0xda as ::core::ffi::c_int {
        count = mpack_expect_native_u16(reader) as uint32_t;
    } else if type_0 as ::core::ffi::c_int == 0xdb as ::core::ffi::c_int {
        count = mpack_expect_native_u32(reader);
    } else {
        mpack_reader_flag_error(reader, mpack_error_type);
        return 0 as uint32_t;
    }
    mpack_reader_flag_if_error(
        reader,
        mpack_track_push(&raw mut (*reader).track, mpack_type_str, count),
    );
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_str_buf(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> size_t {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:523\n%s\nbuf cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = mpack_expect_str(reader) as size_t;
    if mpack_reader_error(reader) as u64 != 0 {
        return 0 as size_t;
    }
    if length > bufsize {
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return 0 as size_t;
    }
    mpack_read_bytes(reader, buf, length);
    if mpack_reader_error(reader) as u64 != 0 {
        return 0 as size_t;
    }
    mpack_done_str(reader);
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_utf8(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> size_t {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:543\n%s\nbuf cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = mpack_expect_str_buf(reader, buf, size);
    if !mpack_utf8_check(buf, length) {
        mpack_reader_flag_error(reader, mpack_error_type);
        return 0 as size_t;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_bin(mut reader: *mut mpack_reader_t) -> uint32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return var.v.l;
    }
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_bin_buf(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> size_t {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:564\n%s\nbuf cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut binsize: size_t = mpack_expect_bin(reader) as size_t;
    if mpack_reader_error(reader) as u64 != 0 {
        return 0 as size_t;
    }
    if binsize > bufsize {
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return 0 as size_t;
    }
    mpack_read_bytes(reader, buf, binsize);
    if mpack_reader_error(reader) as u64 != 0 {
        return 0 as size_t;
    }
    mpack_done_bin(reader);
    return binsize;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_bin_size_buf(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut size: uint32_t,
) {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:581\n%s\nbuf cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_expect_bin_size(reader, size);
    mpack_read_bytes(reader, buf, size as size_t);
    mpack_done_bin(reader);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_ext(
    mut reader: *mut mpack_reader_t,
    mut type_0: *mut int8_t,
) -> uint32_t {
    let mut var: mpack_tag_t = mpack_read_tag(reader);
    if var.type_0 as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *type_0 = mpack_tag_ext_exttype(&raw mut var);
        return mpack_tag_ext_length(&raw mut var);
    }
    *type_0 = 0 as int8_t;
    mpack_reader_flag_error(reader, mpack_error_type);
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_ext_buf(
    mut reader: *mut mpack_reader_t,
    mut type_0: *mut int8_t,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> size_t {
    if buf.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:600\n%s\nbuf cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut extsize: size_t = mpack_expect_ext(reader, type_0) as size_t;
    if mpack_reader_error(reader) as u64 != 0 {
        return 0 as size_t;
    }
    if extsize > bufsize {
        *type_0 = 0 as int8_t;
        mpack_reader_flag_error(reader, mpack_error_too_big);
        return 0 as size_t;
    }
    mpack_read_bytes(reader, buf, extsize);
    if mpack_reader_error(reader) as u64 != 0 {
        *type_0 = 0 as int8_t;
        return 0 as size_t;
    }
    mpack_done_ext(reader);
    return extsize;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_cstr(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) {
    let mut length: uint32_t = mpack_expect_str(reader);
    mpack_read_cstr(reader, buf, bufsize, length as size_t);
    mpack_done_str(reader);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_utf8_cstr(
    mut reader: *mut mpack_reader_t,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) {
    let mut length: uint32_t = mpack_expect_str(reader);
    mpack_read_utf8_cstr(reader, buf, bufsize, length as size_t);
    mpack_done_str(reader);
}
unsafe extern "C" fn mpack_expect_cstr_alloc_unchecked(
    mut reader: *mut mpack_reader_t,
    mut maxsize: size_t,
    mut out_length: *mut size_t,
) -> *mut ::core::ffi::c_char {
    if out_length.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:634\n%s\nout_length cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"out_length != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *out_length = 0 as size_t;
    if maxsize < 1 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-expect.c:639\nmaxsize is zero; you must have room for at least a null-terminator\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_reader_flag_error(reader, mpack_error_bug);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if SIZE_MAX < MPACK_UINT32_MAX as ::core::ffi::c_ulong {
        if maxsize > SIZE_MAX as size_t {
            maxsize = SIZE_MAX as size_t;
        }
    } else if maxsize > MPACK_UINT32_MAX as size_t {
        maxsize = MPACK_UINT32_MAX as size_t;
    }
    let mut length: size_t =
        mpack_expect_str_max(reader, (maxsize as uint32_t).wrapping_sub(1 as uint32_t)) as size_t;
    let mut str: *mut ::core::ffi::c_char =
        mpack_read_bytes_alloc_impl(reader, length, true_0 != 0);
    mpack_done_str(reader);
    if !str.is_null() {
        *out_length = length;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_cstr_alloc(
    mut reader: *mut mpack_reader_t,
    mut maxsize: size_t,
) -> *mut ::core::ffi::c_char {
    let mut length: size_t = 0;
    let mut str: *mut ::core::ffi::c_char =
        mpack_expect_cstr_alloc_unchecked(reader, maxsize, &raw mut length);
    if !str.is_null() && !mpack_str_check_no_null(str, length) {
        test_free(str as *mut ::core::ffi::c_void);
        mpack_reader_flag_error(reader, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_utf8_cstr_alloc(
    mut reader: *mut mpack_reader_t,
    mut maxsize: size_t,
) -> *mut ::core::ffi::c_char {
    let mut length: size_t = 0;
    let mut str: *mut ::core::ffi::c_char =
        mpack_expect_cstr_alloc_unchecked(reader, maxsize, &raw mut length);
    if !str.is_null() && !mpack_utf8_check_no_null(str, length) {
        test_free(str as *mut ::core::ffi::c_void);
        mpack_reader_flag_error(reader, mpack_error_type);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_str_match(
    mut reader: *mut mpack_reader_t,
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    if str.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:689\n%s\nstr cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"str != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if len > MPACK_UINT32_MAX as size_t {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
    mpack_expect_str_length(reader, len as uint32_t);
    if mpack_reader_error(reader) as u64 != 0 {
        return;
    }
    mpack_reader_track_bytes(reader, len as uint32_t as size_t);
    while len > 0 as size_t {
        let fresh0 = str;
        str = str.offset(1);
        if mpack_expect_native_u8(reader) as ::core::ffi::c_int != *fresh0 as ::core::ffi::c_int {
            mpack_reader_flag_error(reader, mpack_error_type);
            return;
        }
        len = len.wrapping_sub(1);
    }
    mpack_done_str(reader);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_tag(
    mut reader: *mut mpack_reader_t,
    mut expected: mpack_tag_t,
) {
    let mut actual: mpack_tag_t = mpack_read_tag(reader);
    if !mpack_tag_equal(actual, expected) {
        mpack_reader_flag_error(reader, mpack_error_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_bin_alloc(
    mut reader: *mut mpack_reader_t,
    mut maxsize: size_t,
    mut size: *mut size_t,
) -> *mut ::core::ffi::c_char {
    if size.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:718\n%s\nsize cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"size != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *size = 0 as size_t;
    if SIZE_MAX < MPACK_UINT32_MAX as ::core::ffi::c_ulong {
        if maxsize > SIZE_MAX as size_t {
            maxsize = SIZE_MAX as size_t;
        }
    } else if maxsize > MPACK_UINT32_MAX as size_t {
        maxsize = MPACK_UINT32_MAX as size_t;
    }
    let mut length: size_t = mpack_expect_bin_max(reader, maxsize as uint32_t) as size_t;
    if mpack_reader_error(reader) as u64 != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut data: *mut ::core::ffi::c_char = mpack_read_bytes_alloc(reader, length);
    mpack_done_bin(reader);
    if !data.is_null() {
        *size = length;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_ext_alloc(
    mut reader: *mut mpack_reader_t,
    mut type_0: *mut int8_t,
    mut maxsize: size_t,
    mut size: *mut size_t,
) -> *mut ::core::ffi::c_char {
    if size.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:744\n%s\nsize cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"size != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *size = 0 as size_t;
    if SIZE_MAX < MPACK_UINT32_MAX as ::core::ffi::c_ulong {
        if maxsize > SIZE_MAX as size_t {
            maxsize = SIZE_MAX as size_t;
        }
    } else if maxsize > MPACK_UINT32_MAX as size_t {
        maxsize = MPACK_UINT32_MAX as size_t;
    }
    let mut length: size_t = mpack_expect_ext_max(reader, type_0, maxsize as uint32_t) as size_t;
    if mpack_reader_error(reader) as u64 != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut data: *mut ::core::ffi::c_char = mpack_read_bytes_alloc(reader, length);
    mpack_done_ext(reader);
    if !data.is_null() {
        *size = length;
    } else {
        *type_0 = 0 as int8_t;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_enum(
    mut reader: *mut mpack_reader_t,
    mut strings: *mut *const ::core::ffi::c_char,
    mut count: size_t,
) -> size_t {
    let mut keylen: size_t = mpack_expect_str(reader) as size_t;
    let mut key: *const ::core::ffi::c_char = mpack_read_bytes_inplace(reader, keylen);
    mpack_done_str(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
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
    mpack_reader_flag_error(reader, mpack_error_type);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_enum_optional(
    mut reader: *mut mpack_reader_t,
    mut strings: *mut *const ::core::ffi::c_char,
    mut count: size_t,
) -> size_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
    if !(count != 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:798\n%s\ncount cannot be zero; no strings are valid!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if strings.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:799\n%s\nstrings cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"strings != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_peek_tag(reader).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_discard(reader);
        return count;
    }
    let mut keylen: size_t = mpack_expect_str(reader) as size_t;
    let mut key: *const ::core::ffi::c_char = mpack_read_bytes_inplace(reader, keylen);
    mpack_done_str(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
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
pub unsafe extern "C" fn mpack_expect_key_uint(
    mut reader: *mut mpack_reader_t,
    mut found: *mut bool,
    mut count: size_t,
) -> size_t {
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
    if count == 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-expect.c:832\ncount cannot be zero; no keys are valid!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_reader_flag_error(reader, mpack_error_bug);
        return count;
    }
    if found.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:836\n%s\nfound cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"found != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_peek_tag(reader).type_0 as ::core::ffi::c_uint
        != mpack_type_uint as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_discard(reader);
        return count;
    }
    let mut value: uint64_t = mpack_expect_u64(reader);
    if mpack_reader_error(reader) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return count;
    }
    if value >= count as uint64_t {
        return count;
    }
    if *found.offset(value as isize) {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        return count;
    }
    *found.offset(value as isize) = true_0 != 0;
    return value as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_expect_key_cstr(
    mut reader: *mut mpack_reader_t,
    mut keys: *mut *const ::core::ffi::c_char,
    mut found: *mut bool,
    mut count: size_t,
) -> size_t {
    let mut i: size_t = mpack_expect_enum_optional(reader, keys, count);
    if i == count {
        return count;
    }
    if found.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-expect.c:871\n%s\nfound cannot be NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"found != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if *found.offset(i as isize) {
        mpack_reader_flag_error(reader, mpack_error_invalid);
        return count;
    }
    *found.offset(i as isize) = true_0 != 0;
    return i;
}
