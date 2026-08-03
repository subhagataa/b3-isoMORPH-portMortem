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
    fn test_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn test_free(p: *mut ::core::ffi::c_void);
    fn test_fwrite(
        ptr: *const ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
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
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn mpack_assert_fail_format(format: *const ::core::ffi::c_char, ...) -> !;
    fn mpack_break_hit_format(format: *const ::core::ffi::c_char, ...);
    fn mpack_realloc(
        old_ptr: *mut ::core::ffi::c_void,
        used_size: size_t,
        new_size: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const MPACK_UINT32_MAX: ::core::ffi::c_uint = UINT32_MAX;
pub const MPACK_PRINT_BYTE_COUNT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mpack_tag_type(mut tag: *mut mpack_tag_t) -> mpack_type_t {
    return (*tag).type_0;
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
#[inline]
unsafe extern "C" fn mpack_tag_bytes(mut tag: *mut mpack_tag_t) -> uint32_t {
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
pub unsafe extern "C" fn mpack_error_to_string(
    mut error: mpack_error_t,
) -> *const ::core::ffi::c_char {
    match error as ::core::ffi::c_uint {
        0 => return b"mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"mpack_error_io\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"mpack_error_invalid\0" as *const u8 as *const ::core::ffi::c_char,
        4 => {
            return b"mpack_error_unsupported\0" as *const u8 as *const ::core::ffi::c_char;
        }
        5 => return b"mpack_error_type\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"mpack_error_too_big\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"mpack_error_memory\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"mpack_error_bug\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"mpack_error_data\0" as *const u8 as *const ::core::ffi::c_char,
        10 => return b"mpack_error_eof\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    if 0 as ::core::ffi::c_int == 0 {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:44\n%s\nunrecognized error %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            error as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return b"(unknown mpack_error_t)\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_type_to_string(
    mut type_0: mpack_type_t,
) -> *const ::core::ffi::c_char {
    match type_0 as ::core::ffi::c_uint {
        0 => return b"mpack_type_missing\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"mpack_type_nil\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"mpack_type_bool\0" as *const u8 as *const ::core::ffi::c_char,
        5 => return b"mpack_type_float\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"mpack_type_double\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"mpack_type_int\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"mpack_type_uint\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"mpack_type_str\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"mpack_type_bin\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"mpack_type_array\0" as *const u8 as *const ::core::ffi::c_char,
        10 => return b"mpack_type_map\0" as *const u8 as *const ::core::ffi::c_char,
        11 => return b"mpack_type_ext\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    if 0 as ::core::ffi::c_int == 0 {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:72\n%s\nunrecognized type %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            type_0 as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return b"(unknown mpack_type_t)\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tag_cmp(
    mut left: mpack_tag_t,
    mut right: mpack_tag_t,
) -> ::core::ffi::c_int {
    if left.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
        && left.v.i >= 0 as int64_t
    {
        left.type_0 = mpack_type_uint;
        left.v.u = left.v.i as uint64_t;
    }
    if right.type_0 as ::core::ffi::c_uint
        == mpack_type_int as ::core::ffi::c_int as ::core::ffi::c_uint
        && right.v.i >= 0 as int64_t
    {
        right.type_0 = mpack_type_uint;
        right.v.u = right.v.i as uint64_t;
    }
    if left.type_0 as ::core::ffi::c_uint != right.type_0 as ::core::ffi::c_uint {
        return if (left.type_0 as ::core::ffi::c_int) < right.type_0 as ::core::ffi::c_int {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
    }
    match left.type_0 as ::core::ffi::c_uint {
        0 | 1 => return 0 as ::core::ffi::c_int,
        2 => return left.v.b as ::core::ffi::c_int - right.v.b as ::core::ffi::c_int,
        3 => {
            if left.v.i == right.v.i {
                return 0 as ::core::ffi::c_int;
            }
            return if left.v.i < right.v.i {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        4 => {
            if left.v.u == right.v.u {
                return 0 as ::core::ffi::c_int;
            }
            return if left.v.u < right.v.u {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        9 | 10 => {
            if left.v.n == right.v.n {
                return 0 as ::core::ffi::c_int;
            }
            return if left.v.n < right.v.n {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        7 | 8 => {
            if left.v.l == right.v.l {
                return 0 as ::core::ffi::c_int;
            }
            return if left.v.l < right.v.l {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        11 => {
            if left.exttype as ::core::ffi::c_int == right.exttype as ::core::ffi::c_int {
                if left.v.l == right.v.l {
                    return 0 as ::core::ffi::c_int;
                }
                return if left.v.l < right.v.l {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                };
            }
            return left.exttype as ::core::ffi::c_int - right.exttype as ::core::ffi::c_int;
        }
        5 => {
            return memcmp(
                &raw mut left.v.f as *const ::core::ffi::c_void,
                &raw mut right.v.f as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            );
        }
        6 => {
            return memcmp(
                &raw mut left.v.d as *const ::core::ffi::c_void,
                &raw mut right.v.d as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            );
        }
        _ => {}
    }
    if 0 as ::core::ffi::c_int == 0 {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:152\n%s\nunrecognized type %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            left.type_0 as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    return false_0;
}
unsafe extern "C" fn mpack_hex_char(mut hex_value: uint8_t) -> ::core::ffi::c_char {
    return (if (hex_value as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
        ('0' as i32 + hex_value as ::core::ffi::c_int) as ::core::ffi::c_char as ::core::ffi::c_int
    } else {
        ('a' as i32 + (hex_value as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
            as ::core::ffi::c_char as ::core::ffi::c_int
    }) as ::core::ffi::c_char;
}
unsafe extern "C" fn mpack_tag_debug_complete_bin_ext(
    mut tag: mpack_tag_t,
    mut string_length: size_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_size: size_t,
) {
    if string_length == 0 as size_t || string_length >= buffer_size {
        return;
    }
    buffer = buffer.offset(string_length as isize);
    buffer_size = buffer_size.wrapping_sub(string_length);
    let mut total: size_t = mpack_tag_bytes(&raw mut tag) as size_t;
    if total == 0 as size_t {
        strncpy(
            buffer,
            b">\0" as *const u8 as *const ::core::ffi::c_char,
            buffer_size,
        );
        return;
    }
    strncpy(
        buffer,
        b": \0" as *const u8 as *const ::core::ffi::c_char,
        buffer_size,
    );
    if buffer_size < 2 as size_t {
        return;
    }
    buffer = buffer.offset(2 as ::core::ffi::c_int as isize);
    buffer_size = buffer_size.wrapping_sub(2 as size_t);
    let mut hex_bytes: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < MPACK_PRINT_BYTE_COUNT as size_t && i < prefix_size && buffer_size > 2 as size_t {
        let mut byte: uint8_t = *prefix.offset(i as isize) as uint8_t;
        *buffer.offset(0 as ::core::ffi::c_int as isize) =
            mpack_hex_char((byte as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t);
        *buffer.offset(1 as ::core::ffi::c_int as isize) =
            mpack_hex_char((byte as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as uint8_t);
        buffer = buffer.offset(2 as ::core::ffi::c_int as isize);
        buffer_size = buffer_size.wrapping_sub(2 as size_t);
        hex_bytes = hex_bytes.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    if buffer_size != 0 as size_t {
        snprintf(
            buffer,
            buffer_size,
            b"%s>\0" as *const u8 as *const ::core::ffi::c_char,
            if total > hex_bytes {
                b"...\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
}
unsafe extern "C" fn mpack_tag_debug_pseudo_json_bin(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_size: size_t,
) {
    if !(mpack_tag_type(&raw mut tag) as ::core::ffi::c_uint
        == mpack_type_bin as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:205\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"mpack_tag_type(&tag) == mpack_type_bin\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = snprintf(
        buffer,
        buffer_size,
        b"<binary data of length %u\0" as *const u8 as *const ::core::ffi::c_char,
        tag.v.l,
    ) as size_t;
    mpack_tag_debug_complete_bin_ext(tag, length, buffer, buffer_size, prefix, prefix_size);
}
unsafe extern "C" fn mpack_tag_debug_pseudo_json_ext(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_size: size_t,
) {
    if !(mpack_tag_type(&raw mut tag) as ::core::ffi::c_uint
        == mpack_type_ext as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:214\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"mpack_tag_type(&tag) == mpack_type_ext\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = snprintf(
        buffer,
        buffer_size,
        b"<ext data of type %i and length %u\0" as *const u8 as *const ::core::ffi::c_char,
        mpack_tag_ext_exttype(&raw mut tag) as ::core::ffi::c_int,
        mpack_tag_ext_length(&raw mut tag),
    ) as size_t;
    mpack_tag_debug_complete_bin_ext(tag, length, buffer, buffer_size, prefix, prefix_size);
}
unsafe extern "C" fn mpack_tag_debug_pseudo_json_impl(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_size: size_t,
) {
    match tag.type_0 as ::core::ffi::c_uint {
        0 => {
            snprintf(
                buffer,
                buffer_size,
                b"<missing!>\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        1 => {
            snprintf(
                buffer,
                buffer_size,
                b"null\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        2 => {
            snprintf(
                buffer,
                buffer_size,
                if tag.v.b as ::core::ffi::c_int != 0 {
                    b"true\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"false\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return;
        }
        3 => {
            snprintf(
                buffer,
                buffer_size,
                b"%li\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.i,
            );
            return;
        }
        4 => {
            snprintf(
                buffer,
                buffer_size,
                b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.u,
            );
            return;
        }
        5 => {
            snprintf(
                buffer,
                buffer_size,
                b"%f\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.f as ::core::ffi::c_double,
            );
            return;
        }
        6 => {
            snprintf(
                buffer,
                buffer_size,
                b"%f\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.d,
            );
            return;
        }
        7 => {
            snprintf(
                buffer,
                buffer_size,
                b"<string of %u bytes>\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.l,
            );
            return;
        }
        8 => {
            mpack_tag_debug_pseudo_json_bin(tag, buffer, buffer_size, prefix, prefix_size);
            return;
        }
        11 => {
            mpack_tag_debug_pseudo_json_ext(tag, buffer, buffer_size, prefix, prefix_size);
            return;
        }
        9 => {
            snprintf(
                buffer,
                buffer_size,
                b"<array of %u elements>\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.n,
            );
            return;
        }
        10 => {
            snprintf(
                buffer,
                buffer_size,
                b"<map of %u key-value pairs>\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.n,
            );
            return;
        }
        _ => {}
    }
    snprintf(
        buffer,
        buffer_size,
        b"<unknown!>\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tag_debug_pseudo_json(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_size: size_t,
) {
    if !(buffer_size > 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:281\n%s\nbuffer size cannot be zero!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *buffer.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    mpack_tag_debug_pseudo_json_impl(tag, buffer, buffer_size, prefix, prefix_size);
    *buffer.offset(buffer_size.wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
}
unsafe extern "C" fn mpack_tag_debug_describe_impl(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) {
    match tag.type_0 as ::core::ffi::c_uint {
        0 => {
            snprintf(
                buffer,
                buffer_size,
                b"missing\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        1 => {
            snprintf(
                buffer,
                buffer_size,
                b"nil\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        2 => {
            snprintf(
                buffer,
                buffer_size,
                if tag.v.b as ::core::ffi::c_int != 0 {
                    b"true\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"false\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return;
        }
        3 => {
            snprintf(
                buffer,
                buffer_size,
                b"int %li\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.i,
            );
            return;
        }
        4 => {
            snprintf(
                buffer,
                buffer_size,
                b"uint %lu\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.u,
            );
            return;
        }
        5 => {
            snprintf(
                buffer,
                buffer_size,
                b"float %f\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.f as ::core::ffi::c_double,
            );
            return;
        }
        6 => {
            snprintf(
                buffer,
                buffer_size,
                b"double %f\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.d,
            );
            return;
        }
        7 => {
            snprintf(
                buffer,
                buffer_size,
                b"str of %u bytes\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.l,
            );
            return;
        }
        8 => {
            snprintf(
                buffer,
                buffer_size,
                b"bin of %u bytes\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.l,
            );
            return;
        }
        11 => {
            snprintf(
                buffer,
                buffer_size,
                b"ext of type %i, %u bytes\0" as *const u8 as *const ::core::ffi::c_char,
                mpack_tag_ext_exttype(&raw mut tag) as ::core::ffi::c_int,
                mpack_tag_ext_length(&raw mut tag),
            );
            return;
        }
        9 => {
            snprintf(
                buffer,
                buffer_size,
                b"array of %u elements\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.n,
            );
            return;
        }
        10 => {
            snprintf(
                buffer,
                buffer_size,
                b"map of %u key-value pairs\0" as *const u8 as *const ::core::ffi::c_char,
                tag.v.n,
            );
            return;
        }
        _ => {}
    }
    snprintf(
        buffer,
        buffer_size,
        b"unknown!\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_tag_debug_describe(
    mut tag: mpack_tag_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) {
    if !(buffer_size > 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:346\n%s\nbuffer size cannot be zero!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *buffer.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    mpack_tag_debug_describe_impl(tag, buffer, buffer_size);
    *buffer.offset(buffer_size.wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_init(mut track: *mut mpack_track_t) -> mpack_error_t {
    (*track).count = 0 as size_t;
    (*track).capacity = MPACK_TRACKING_INITIAL_CAPACITY as size_t;
    (*track).elements = test_malloc(
        (::core::mem::size_of::<mpack_track_element_t>() as size_t).wrapping_mul((*track).capacity),
    ) as *mut mpack_track_element_t;
    if (*track).elements.is_null() {
        return mpack_error_memory;
    }
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_grow(mut track: *mut mpack_track_t) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:377\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*track).count == (*track).capacity) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:378\n%s\nincorrect growing?\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->count == track->capacity\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut new_capacity: size_t = (*track).capacity.wrapping_mul(2 as size_t);
    let mut new_elements: *mut mpack_track_element_t = mpack_realloc(
        (*track).elements as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<mpack_track_element_t>() as size_t).wrapping_mul((*track).count),
        (::core::mem::size_of::<mpack_track_element_t>() as size_t).wrapping_mul(new_capacity),
    ) as *mut mpack_track_element_t;
    if new_elements.is_null() {
        return mpack_error_memory;
    }
    (*track).elements = new_elements;
    (*track).capacity = new_capacity;
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_push(
    mut track: *mut mpack_track_t,
    mut type_0: mpack_type_t,
    mut count: uint32_t,
) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:393\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*track).count == (*track).capacity {
        let mut error: mpack_error_t = mpack_track_grow(track);
        if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
            return error;
        }
    }
    (*(*track).elements.offset((*track).count as isize)).type_0 = type_0;
    (*(*track).elements.offset((*track).count as isize)).left = count;
    (*(*track).elements.offset((*track).count as isize)).builder = false_0 != 0;
    (*(*track).elements.offset((*track).count as isize)).key_needs_value = false_0 != 0;
    (*track).count = (*track).count.wrapping_add(1);
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_push_builder(
    mut track: *mut mpack_track_t,
    mut type_0: mpack_type_t,
) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:414\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*track).count == (*track).capacity {
        let mut error: mpack_error_t = mpack_track_grow(track);
        if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
            return error;
        }
    }
    (*(*track).elements.offset((*track).count as isize)).type_0 = type_0;
    (*(*track).elements.offset((*track).count as isize)).left = 0 as uint32_t;
    (*(*track).elements.offset((*track).count as isize)).builder = true_0 != 0;
    (*(*track).elements.offset((*track).count as isize)).key_needs_value = false_0 != 0;
    (*track).count = (*track).count.wrapping_add(1);
    return mpack_ok;
}
unsafe extern "C" fn mpack_track_pop_impl(
    mut track: *mut mpack_track_t,
    mut type_0: mpack_type_t,
    mut builder: bool,
) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:434\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*track).count == 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:438\nattempting to close a %s but nothing was opened!\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_type_to_string(type_0),
        );
        return mpack_error_bug;
    }
    let mut element: *mut mpack_track_element_t = (*track)
        .elements
        .offset((*track).count.wrapping_sub(1 as size_t) as isize)
        as *mut mpack_track_element_t;
    if (*element).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:446\nattempting to close a %s but the open element is a %s!\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_type_to_string(type_0),
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    if (*element).key_needs_value {
        if !(type_0 as ::core::ffi::c_uint
            == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            mpack_assert_fail_format(
                b"mpack assertion failed at src/mpack/mpack-common.c:451\n%s\nkey_needs_value can only be true for maps!\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"type == mpack_type_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else {
        };
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:453\nattempting to close a %s but an odd number of elements were written\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_type_to_string(type_0),
        );
        return mpack_error_bug;
    }
    if (*element).left != 0 as uint32_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:460\nattempting to close a %s but there are %i %s left\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_type_to_string(type_0),
            (*element).left,
            if type_0 as ::core::ffi::c_uint
                == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
                || type_0 as ::core::ffi::c_uint
                    == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                b"elements\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"bytes\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return mpack_error_bug;
    }
    if (*element).builder as ::core::ffi::c_int != builder as ::core::ffi::c_int {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:467\nattempting to pop a %sbuilder but the open element is %sa builder\0"
                as *const u8 as *const ::core::ffi::c_char,
            if builder as ::core::ffi::c_int != 0 {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"non-\0" as *const u8 as *const ::core::ffi::c_char
            },
            if (*element).builder as ::core::ffi::c_int != 0 {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"not \0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return mpack_error_bug;
    }
    (*track).count = (*track).count.wrapping_sub(1);
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_pop(
    mut track: *mut mpack_track_t,
    mut type_0: mpack_type_t,
) -> mpack_error_t {
    return mpack_track_pop_impl(track, type_0, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_pop_builder(
    mut track: *mut mpack_track_t,
    mut type_0: mpack_type_t,
) -> mpack_error_t {
    return mpack_track_pop_impl(track, type_0, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_peek_element(
    mut track: *mut mpack_track_t,
    mut read: bool,
) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:485\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*track).count == 0 as size_t {
        return mpack_ok;
    }
    let mut element: *mut mpack_track_element_t = (*track)
        .elements
        .offset((*track).count.wrapping_sub(1 as size_t) as isize)
        as *mut mpack_track_element_t;
    if (*element).type_0 as ::core::ffi::c_uint
        != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*element).type_0 as ::core::ffi::c_uint
            != mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:495\nelements cannot be %s within an %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"read\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"written\0" as *const u8 as *const ::core::ffi::c_char
            },
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    if !(*element).builder && (*element).left == 0 as uint32_t && !(*element).key_needs_value {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:501\ntoo many elements %s for %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"read\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"written\0" as *const u8 as *const ::core::ffi::c_char
            },
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_element(
    mut track: *mut mpack_track_t,
    mut read: bool,
) -> mpack_error_t {
    let mut error: mpack_error_t = mpack_track_peek_element(track, read);
    if (*track).count == 0 as size_t
        || error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return error;
    }
    let mut element: *mut mpack_track_element_t = (*track)
        .elements
        .offset((*track).count.wrapping_sub(1 as size_t) as isize)
        as *mut mpack_track_element_t;
    if (*element).type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !(*element).key_needs_value {
            (*element).key_needs_value = true_0 != 0;
            return mpack_ok;
        }
        (*element).key_needs_value = false_0 != 0;
    }
    if !(*element).builder {
        (*element).left = (*element).left.wrapping_sub(1);
    }
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_bytes(
    mut track: *mut mpack_track_t,
    mut read: bool,
    mut count: size_t,
) -> mpack_error_t {
    if (*track).elements.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-common.c:530\n%s\nnull track elements!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"track->elements\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if count > MPACK_UINT32_MAX as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:534\n%s more bytes than could possibly fit in a str/bin/ext!\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"reading\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"writing\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return mpack_error_bug;
    }
    if (*track).count == 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:539\nbytes cannot be %s with no open bin, str or ext\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"read\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"written\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return mpack_error_bug;
    }
    let mut element: *mut mpack_track_element_t = (*track)
        .elements
        .offset((*track).count.wrapping_sub(1 as size_t) as isize)
        as *mut mpack_track_element_t;
    if (*element).type_0 as ::core::ffi::c_uint
        == mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*element).type_0 as ::core::ffi::c_uint
            == mpack_type_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:547\nbytes cannot be %s within an %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"read\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"written\0" as *const u8 as *const ::core::ffi::c_char
            },
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    if ((*element).left as size_t) < count {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:553\ntoo many bytes %s for %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            if read as ::core::ffi::c_int != 0 {
                b"read\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"written\0" as *const u8 as *const ::core::ffi::c_char
            },
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    (*element).left = (*element).left.wrapping_sub(count as uint32_t);
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_str_bytes_all(
    mut track: *mut mpack_track_t,
    mut read: bool,
    mut count: size_t,
) -> mpack_error_t {
    let mut error: mpack_error_t = mpack_track_bytes(track, read, count);
    if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        return error;
    }
    let mut element: *mut mpack_track_element_t = (*track)
        .elements
        .offset((*track).count.wrapping_sub(1 as size_t) as isize)
        as *mut mpack_track_element_t;
    if (*element).type_0 as ::core::ffi::c_uint
        != mpack_type_str as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:569\nthe open type must be a string, not a %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_type_to_string((*element).type_0),
        );
        return mpack_error_bug;
    }
    if (*element).left != 0 as uint32_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:574\nnot all bytes were read; the wrong byte count was requested for a string read.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return mpack_error_bug;
    }
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_check_empty(mut track: *mut mpack_track_t) -> mpack_error_t {
    if (*track).count != 0 as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-common.c:583\nunclosed %s\0" as *const u8
                as *const ::core::ffi::c_char,
            mpack_type_to_string(
                (*(*track).elements.offset(0 as ::core::ffi::c_int as isize)).type_0,
            ),
        );
        return mpack_error_bug;
    }
    return mpack_ok;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_track_destroy(
    mut track: *mut mpack_track_t,
    mut cancel: bool,
) -> mpack_error_t {
    let mut error: mpack_error_t = (if cancel as ::core::ffi::c_int != 0 {
        mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    } else {
        mpack_track_check_empty(track) as ::core::ffi::c_uint
    }) as mpack_error_t;
    if !(*track).elements.is_null() {
        test_free((*track).elements as *mut ::core::ffi::c_void);
        (*track).elements = ::core::ptr::null_mut::<mpack_track_element_t>();
    }
    return error;
}
unsafe extern "C" fn mpack_utf8_check_impl(
    mut str: *const uint8_t,
    mut count: size_t,
    mut allow_null: bool,
) -> bool {
    while count > 0 as size_t {
        let mut lead: uint8_t = *str.offset(0 as ::core::ffi::c_int as isize);
        if !allow_null && lead as ::core::ffi::c_int == '\0' as i32 {
            return false_0 != 0;
        }
        if lead as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int {
            str = str.offset(1);
            count = count.wrapping_sub(1);
        } else if lead as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
            == 0xc0 as ::core::ffi::c_int
        {
            if count < 2 as size_t {
                return false_0 != 0;
            }
            let mut cont: uint8_t = *str.offset(1 as ::core::ffi::c_int as isize);
            if cont as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            str = str.offset(2 as ::core::ffi::c_int as isize);
            count = count.wrapping_sub(2 as size_t);
            let mut z: uint32_t = ((lead as ::core::ffi::c_int & !(0xe0 as ::core::ffi::c_int))
                as uint32_t)
                << 6 as ::core::ffi::c_int
                | (cont as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t;
            if z < 0x80 as uint32_t {
                return false_0 != 0;
            }
        } else if lead as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
            == 0xe0 as ::core::ffi::c_int
        {
            if count < 3 as size_t {
                return false_0 != 0;
            }
            let mut cont1: uint8_t = *str.offset(1 as ::core::ffi::c_int as isize);
            if cont1 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            let mut cont2: uint8_t = *str.offset(2 as ::core::ffi::c_int as isize);
            if cont2 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            str = str.offset(3 as ::core::ffi::c_int as isize);
            count = count.wrapping_sub(3 as size_t);
            let mut z_0: uint32_t = ((lead as ::core::ffi::c_int & !(0xf0 as ::core::ffi::c_int))
                as uint32_t)
                << 12 as ::core::ffi::c_int
                | ((cont1 as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t)
                    << 6 as ::core::ffi::c_int
                | (cont2 as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t;
            if z_0 < 0x800 as uint32_t {
                return false_0 != 0;
            }
            if z_0 >= 0xd800 as uint32_t && z_0 <= 0xdfff as uint32_t {
                return false_0 != 0;
            }
        } else if lead as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xf0 as ::core::ffi::c_int
        {
            if count < 4 as size_t {
                return false_0 != 0;
            }
            let mut cont1_0: uint8_t = *str.offset(1 as ::core::ffi::c_int as isize);
            if cont1_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            let mut cont2_0: uint8_t = *str.offset(2 as ::core::ffi::c_int as isize);
            if cont2_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            let mut cont3: uint8_t = *str.offset(3 as ::core::ffi::c_int as isize);
            if cont3 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            str = str.offset(4 as ::core::ffi::c_int as isize);
            count = count.wrapping_sub(4 as size_t);
            let mut z_1: uint32_t = ((lead as ::core::ffi::c_int & !(0xf8 as ::core::ffi::c_int))
                as uint32_t)
                << 18 as ::core::ffi::c_int
                | ((cont1_0 as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t)
                    << 12 as ::core::ffi::c_int
                | ((cont2_0 as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t)
                    << 6 as ::core::ffi::c_int
                | (cont3 as ::core::ffi::c_int & !(0xc0 as ::core::ffi::c_int)) as uint32_t;
            if z_1 < 0x10000 as uint32_t {
                return false_0 != 0;
            }
            if z_1 > 0x10ffff as uint32_t {
                return false_0 != 0;
            }
        } else {
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_utf8_check(
    mut str: *const ::core::ffi::c_char,
    mut bytes: size_t,
) -> bool {
    return mpack_utf8_check_impl(str as *const uint8_t, bytes, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_utf8_check_no_null(
    mut str: *const ::core::ffi::c_char,
    mut bytes: size_t,
) -> bool {
    return mpack_utf8_check_impl(str as *const uint8_t, bytes, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_str_check_no_null(
    mut str: *const ::core::ffi::c_char,
    mut bytes: size_t,
) -> bool {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < bytes {
        if *str.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
            return false_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_append(
    mut print: *mut mpack_print_t,
    mut data: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    let mut copy: size_t = (*print).size.wrapping_sub((*print).count);
    if copy > count {
        copy = count;
    }
    memcpy(
        (*print).buffer.offset((*print).count as isize) as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        copy,
    );
    (*print).count = (*print).count.wrapping_add(copy);
    data = data.offset(copy as isize);
    count = count.wrapping_sub(copy);
    if count == 0 as size_t || (*print).callback.is_none() {
        return;
    }
    (*print).callback.expect("non-null function pointer")(
        (*print).context,
        (*print).buffer,
        (*print).count,
    );
    if count > (*print).size.wrapping_div(2 as size_t) {
        (*print).count = 0 as size_t;
        (*print).callback.expect("non-null function pointer")((*print).context, data, count);
    } else {
        memcpy(
            (*print).buffer as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            count,
        );
        (*print).count = count;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_flush(mut print: *mut mpack_print_t) {
    if (*print).count > 0 as size_t && (*print).callback.is_some() {
        (*print).callback.expect("non-null function pointer")(
            (*print).context,
            (*print).buffer,
            (*print).count,
        );
        (*print).count = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_print_file_callback(
    mut context: *mut ::core::ffi::c_void,
    mut data: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    let mut file: *mut FILE = context as *mut FILE;
    test_fwrite(data as *const ::core::ffi::c_void, 1 as size_t, count, file);
}
pub const MPACK_TRACKING_INITIAL_CAPACITY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
