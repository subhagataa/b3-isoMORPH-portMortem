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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mpack_assert_fail_format(format: *const ::core::ffi::c_char, ...) -> !;
    fn mpack_break_hit_format(format: *const ::core::ffi::c_char, ...);
    fn mpack_realloc(
        old_ptr: *mut ::core::ffi::c_void,
        used_size: size_t,
        new_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mpack_track_init(track: *mut mpack_track_t) -> mpack_error_t;
    fn mpack_track_push(
        track: *mut mpack_track_t,
        type_0: mpack_type_t,
        count: uint32_t,
    ) -> mpack_error_t;
    fn mpack_track_push_builder(track: *mut mpack_track_t, type_0: mpack_type_t) -> mpack_error_t;
    fn mpack_track_pop(track: *mut mpack_track_t, type_0: mpack_type_t) -> mpack_error_t;
    fn mpack_track_pop_builder(track: *mut mpack_track_t, type_0: mpack_type_t) -> mpack_error_t;
    fn mpack_track_element(track: *mut mpack_track_t, read: bool) -> mpack_error_t;
    fn mpack_track_bytes(track: *mut mpack_track_t, read: bool, count: size_t) -> mpack_error_t;
    fn mpack_track_check_empty(track: *mut mpack_track_t) -> mpack_error_t;
    fn mpack_track_destroy(track: *mut mpack_track_t, cancel: bool) -> mpack_error_t;
    fn mpack_utf8_check(str: *const ::core::ffi::c_char, bytes: size_t) -> bool;
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
pub struct mpack_growable_writer_t {
    pub target_data: *mut *mut ::core::ffi::c_char,
    pub target_size: *mut size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT8_MIN: ::core::ffi::c_int = -(128 as ::core::ffi::c_int);
pub const INT16_MIN: ::core::ffi::c_int = -(32767 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const MPACK_INT8_MIN: ::core::ffi::c_int = INT8_MIN;
pub const MPACK_INT16_MIN: ::core::ffi::c_int = INT16_MIN;
pub const MPACK_INT32_MIN: ::core::ffi::c_int = INT32_MIN;
pub const MPACK_UINT8_MAX: ::core::ffi::c_int = UINT8_MAX;
pub const MPACK_UINT16_MAX: ::core::ffi::c_int = UINT16_MAX;
pub const MPACK_UINT32_MAX: ::core::ffi::c_uint = UINT32_MAX;
pub const MPACK_MAXIMUM_TAG_SIZE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPACK_TIMESTAMP_NANOSECONDS_MAX: ::core::ffi::c_int = 999999999 as ::core::ffi::c_int;
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
unsafe extern "C" fn mpack_store_u8(p: *mut ::core::ffi::c_char, val: uint8_t) {
    // SAFETY: caller (C-ABI/FFI contract) guarantees `p` points to at least
    // 1 valid, writable byte.
    unsafe {
        core::slice::from_raw_parts_mut(p as *mut u8, 1).copy_from_slice(&val.to_be_bytes());
    }
}
#[inline]
unsafe extern "C" fn mpack_store_u16(p: *mut ::core::ffi::c_char, val: uint16_t) {
    let bytes = val.to_be_bytes();
    // SAFETY: caller (C-ABI/FFI contract) guarantees `p` points to at least
    // bytes.len() valid, writable bytes.
    unsafe {
        core::slice::from_raw_parts_mut(p as *mut u8, bytes.len()).copy_from_slice(&bytes);
    }
}
#[inline]
unsafe extern "C" fn mpack_store_u32(p: *mut ::core::ffi::c_char, val: uint32_t) {
    let bytes = val.to_be_bytes();
    // SAFETY: caller (C-ABI/FFI contract) guarantees `p` points to at least
    // bytes.len() valid, writable bytes.
    unsafe {
        core::slice::from_raw_parts_mut(p as *mut u8, bytes.len()).copy_from_slice(&bytes);
    }
}
#[inline]
unsafe extern "C" fn mpack_store_u64(p: *mut ::core::ffi::c_char, val: uint64_t) {
    let bytes = val.to_be_bytes();
    // SAFETY: caller (C-ABI/FFI contract) guarantees `p` points to at least
    // bytes.len() valid, writable bytes.
    unsafe {
        core::slice::from_raw_parts_mut(p as *mut u8, bytes.len()).copy_from_slice(&bytes);
    }
}
#[inline]
unsafe extern "C" fn mpack_store_i8(mut p: *mut ::core::ffi::c_char, mut val: int8_t) {
    mpack_store_u8(p, val as uint8_t);
}
#[inline]
unsafe extern "C" fn mpack_store_i16(mut p: *mut ::core::ffi::c_char, mut val: int16_t) {
    mpack_store_u16(p, val as uint16_t);
}
#[inline]
unsafe extern "C" fn mpack_store_i32(mut p: *mut ::core::ffi::c_char, mut val: int32_t) {
    mpack_store_u32(p, val as uint32_t);
}
#[inline]
unsafe extern "C" fn mpack_store_i64(mut p: *mut ::core::ffi::c_char, mut val: int64_t) {
    mpack_store_u64(p, val as uint64_t);
}
#[inline]
unsafe extern "C" fn mpack_store_float(
    p: *mut ::core::ffi::c_char,
    value: ::core::ffi::c_float,
) {
    // Uses Rust's safe f32::to_bits() instead of a raw union reinterpret cast.
    unsafe { mpack_store_u32(p, value.to_bits()) };
}
#[inline]
unsafe extern "C" fn mpack_store_double(
    p: *mut ::core::ffi::c_char,
    value: ::core::ffi::c_double,
) {
    // Uses Rust's safe f64::to_bits() instead of a raw union reinterpret cast.
    unsafe { mpack_store_u64(p, value.to_bits()) };
}
pub const MPACK_TAG_SIZE_FIXSTR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_STR8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_FIXEXT8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MPACK_TAG_SIZE_EXT8: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MPACK_WRITER_MINIMUM_BUFFER_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mpack_writer_set_context(
    mut writer: *mut mpack_writer_t,
    mut context: *mut ::core::ffi::c_void,
) {
    (*writer).context = context;
}
#[inline]
unsafe extern "C" fn mpack_writer_set_teardown(
    mut writer: *mut mpack_writer_t,
    mut teardown: mpack_writer_teardown_t,
) {
    (*writer).teardown = teardown;
}
#[inline]
unsafe extern "C" fn mpack_writer_buffer_used(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).position.offset_from((*writer).buffer) as ::core::ffi::c_long as size_t;
}
#[inline]
unsafe extern "C" fn mpack_writer_buffer_left(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).end.offset_from((*writer).position) as ::core::ffi::c_long as size_t;
}
#[inline]
unsafe extern "C" fn mpack_writer_buffer_size(mut writer: *mut mpack_writer_t) -> size_t {
    return (*writer).end.offset_from((*writer).buffer) as ::core::ffi::c_long as size_t;
}
#[inline]
unsafe extern "C" fn mpack_writer_error(mut writer: *mut mpack_writer_t) -> mpack_error_t {
    return (*writer).error;
}
#[inline]
unsafe extern "C" fn mpack_write_int(mut writer: *mut mpack_writer_t, mut value: int64_t) {
    mpack_write_i64(writer, value);
}
#[inline]
unsafe extern "C" fn mpack_write_uint(mut writer: *mut mpack_writer_t, mut value: uint64_t) {
    mpack_write_u64(writer, value);
}
#[inline]
unsafe extern "C" fn mpack_builder_compound_push(mut writer: *mut mpack_writer_t) {
    let mut build: *mut mpack_build_t = (*writer).builder.current_build;
    if !build.is_null() {
        (*build).nested_compound_elements = (*build).nested_compound_elements.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn mpack_finish_bin(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_bin);
}
#[inline]
unsafe extern "C" fn mpack_finish_ext(mut writer: *mut mpack_writer_t) {
    mpack_writer_track_pop(writer, mpack_type_ext);
}
unsafe extern "C" fn mpack_writer_flag_if_error(
    mut writer: *mut mpack_writer_t,
    mut error: mpack_error_t,
) {
    if error as ::core::ffi::c_uint != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        mpack_writer_flag_error(writer, error);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_track_push(
    writer: *mut mpack_writer_t,
    type_0: mpack_type_t,
    count: uint32_t,
) {
    // SAFETY: FFI contract requires `writer` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let writer_ref: &mut mpack_writer_t = unsafe { &mut *writer };
    if writer_ref.error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let track_ptr: *mut mpack_track_t = &mut writer_ref.track;
        unsafe {
            mpack_writer_flag_if_error(writer, mpack_track_push(track_ptr, type_0, count));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_track_push_builder(
    writer: *mut mpack_writer_t,
    type_0: mpack_type_t,
) {
    // SAFETY: FFI contract requires `writer` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let writer_ref: &mut mpack_writer_t = unsafe { &mut *writer };
    if writer_ref.error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let track_ptr: *mut mpack_track_t = &mut writer_ref.track;
        unsafe {
            mpack_writer_flag_if_error(writer, mpack_track_push_builder(track_ptr, type_0));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_track_pop(
    writer: *mut mpack_writer_t,
    type_0: mpack_type_t,
) {
    // SAFETY: FFI contract requires `writer` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let writer_ref: &mut mpack_writer_t = unsafe { &mut *writer };
    if writer_ref.error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let track_ptr: *mut mpack_track_t = &mut writer_ref.track;
        unsafe {
            mpack_writer_flag_if_error(writer, mpack_track_pop(track_ptr, type_0));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_track_pop_builder(
    writer: *mut mpack_writer_t,
    type_0: mpack_type_t,
) {
    // SAFETY: FFI contract requires `writer` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let writer_ref: &mut mpack_writer_t = unsafe { &mut *writer };
    if writer_ref.error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let track_ptr: *mut mpack_track_t = &mut writer_ref.track;
        unsafe {
            mpack_writer_flag_if_error(writer, mpack_track_pop_builder(track_ptr, type_0));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_track_bytes(
    writer: *mut mpack_writer_t,
    count: size_t,
) {
    // SAFETY: FFI contract requires `writer` to be a valid, non-null,
    // aligned pointer for the duration of this call.
    let writer_ref: &mut mpack_writer_t = unsafe { &mut *writer };
    if writer_ref.error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let track_ptr: *mut mpack_track_t = &mut writer_ref.track;
        unsafe {
            mpack_writer_flag_if_error(writer, mpack_track_bytes(track_ptr, false_0 != 0, count));
        }
    }
}
#[inline]
unsafe extern "C" fn mpack_writer_track_element(mut writer: *mut mpack_writer_t) {
    if (*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_writer_flag_if_error(
            writer,
            mpack_track_element(&raw mut (*writer).track, false_0 != 0),
        );
    }
    if !(*writer).builder.current_build.is_null() {
        let mut build: *mut mpack_build_t = (*writer).builder.current_build;
        if (*build).nested_compound_elements == 0 as uint32_t {
            if (*build).type_0 as ::core::ffi::c_uint
                != mpack_type_map as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*build).count = (*build).count.wrapping_add(1);
            } else if (*build).key_needs_value {
                (*build).key_needs_value = false_0 != 0;
                (*build).count = (*build).count.wrapping_add(1);
            } else {
                (*build).key_needs_value = true_0 != 0;
            }
        }
    }
}
unsafe extern "C" fn mpack_writer_clear(mut writer: *mut mpack_writer_t) {
    (*writer).version = mpack_version_current;
    (*writer).flush = None;
    (*writer).error_fn = None;
    (*writer).teardown = None;
    (*writer).context = NULL;
    (*writer).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).position = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).error = mpack_ok;
    memset(
        &raw mut (*writer).track as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mpack_track_t>() as size_t,
    );
    (*writer).builder.current_build = ::core::ptr::null_mut::<mpack_build_t>();
    (*writer).builder.latest_build = ::core::ptr::null_mut::<mpack_build_t>();
    (*writer).builder.current_page = ::core::ptr::null_mut::<mpack_builder_page_t>();
    (*writer).builder.pages = ::core::ptr::null_mut::<mpack_builder_page_t>();
    (*writer).builder.stash_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).builder.stash_position = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).builder.stash_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_init(
    mut writer: *mut mpack_writer_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    if buffer.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:125\n%s\ncannot initialize writer with empty buffer\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"buffer != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_writer_clear(writer);
    (*writer).buffer = buffer;
    (*writer).position = buffer;
    (*writer).end = (*writer).buffer.offset(size as isize);
    mpack_writer_flag_if_error(writer, mpack_track_init(&raw mut (*writer).track));
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_init_error(
    mut writer: *mut mpack_writer_t,
    mut error: mpack_error_t,
) {
    mpack_writer_clear(writer);
    (*writer).error = error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_set_flush(
    mut writer: *mut mpack_writer_t,
    mut flush: mpack_writer_flush_t,
) {
    if mpack_writer_buffer_size(writer) < MPACK_WRITER_MINIMUM_BUFFER_SIZE as size_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:155\nbuffer size is %i, but minimum buffer size for flush is %i\0"
                as *const u8 as *const ::core::ffi::c_char,
            mpack_writer_buffer_size(writer) as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    (*writer).flush = flush;
}
unsafe extern "C" fn mpack_writer_get_reserved(
    mut writer: *mut mpack_writer_t,
) -> *mut ::core::ffi::c_char {
    return &raw mut (*writer).reserved as *mut *mut ::core::ffi::c_void
        as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn mpack_growable_writer_flush(
    mut writer: *mut mpack_writer_t,
    mut data: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if data == (*writer).buffer as *const ::core::ffi::c_char {
        if mpack_writer_buffer_used(writer) == count {
            return;
        }
        (*writer).position = (*writer).buffer.offset(count as isize);
        count = 0 as size_t;
    }
    let mut used: size_t = mpack_writer_buffer_used(writer);
    let mut size: size_t = mpack_writer_buffer_size(writer);
    if !(data == (*writer).buffer as *const ::core::ffi::c_char || used.wrapping_add(count) > size)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:210\n%s\nextra flush for %i but there is %i space left in the buffer! (%i/%i)\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"data == writer->buffer || used + count > size\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            mpack_writer_buffer_left(writer) as ::core::ffi::c_int,
            used as ::core::ffi::c_int,
            size as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut new_size: size_t = size.wrapping_mul(2 as size_t);
    while new_size < used.wrapping_add(count) {
        new_size = new_size.wrapping_mul(2 as size_t);
    }
    let mut new_buffer: *mut ::core::ffi::c_char =
        mpack_realloc((*writer).buffer as *mut ::core::ffi::c_void, used, new_size)
            as *mut ::core::ffi::c_char;
    if new_buffer.is_null() {
        mpack_writer_flag_error(writer, mpack_error_memory);
        return;
    }
    (*writer).position = new_buffer.offset(used as isize);
    (*writer).buffer = new_buffer;
    (*writer).end = (*writer).buffer.offset(new_size as isize);
    if count > 0 as size_t {
        memcpy(
            (*writer).position as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            count,
        );
        (*writer).position = (*writer).position.offset(count as isize);
    }
}
unsafe extern "C" fn mpack_growable_writer_teardown(mut writer: *mut mpack_writer_t) {
    let mut growable_writer: *mut mpack_growable_writer_t =
        mpack_writer_get_reserved(writer) as *mut mpack_growable_writer_t;
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if mpack_writer_buffer_used(writer)
            < mpack_writer_buffer_size(writer).wrapping_div(2 as size_t)
        {
            let mut used: size_t = mpack_writer_buffer_used(writer);
            let mut size: size_t = if used != 0 as size_t {
                used
            } else {
                1 as size_t
            };
            let mut buffer: *mut ::core::ffi::c_char =
                mpack_realloc((*writer).buffer as *mut ::core::ffi::c_void, used, size)
                    as *mut ::core::ffi::c_char;
            if buffer.is_null() {
                test_free((*writer).buffer as *mut ::core::ffi::c_void);
                mpack_writer_flag_error(writer, mpack_error_memory);
                return;
            }
            (*writer).buffer = buffer;
            (*writer).position = (*writer).buffer.offset(used as isize);
            (*writer).end = (*writer).position;
        }
        *(*growable_writer).target_data = (*writer).buffer;
        *(*growable_writer).target_size = mpack_writer_buffer_used(writer);
        (*writer).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else if !(*writer).buffer.is_null() {
        test_free((*writer).buffer as *mut ::core::ffi::c_void);
        (*writer).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*writer).context = NULL;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_init_growable(
    mut writer: *mut mpack_writer_t,
    mut target_data: *mut *mut ::core::ffi::c_char,
    mut target_size: *mut size_t,
) {
    if target_data.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:277\n%s\ncannot initialize writer without a destination for the data\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"target_data != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if target_size.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:278\n%s\ncannot initialize writer without a destination for the size\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"target_size != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    *target_data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *target_size = 0 as size_t;
    let mut growable_writer: *mut mpack_growable_writer_t =
        mpack_writer_get_reserved(writer) as *mut mpack_growable_writer_t;
    (*growable_writer).target_data = target_data;
    (*growable_writer).target_size = target_size;
    let mut capacity: size_t = MPACK_BUFFER_SIZE as size_t;
    let mut buffer: *mut ::core::ffi::c_char = test_malloc(capacity) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        mpack_writer_init_error(writer, mpack_error_memory);
        return;
    }
    mpack_writer_init(writer, buffer, capacity);
    mpack_writer_set_flush(
        writer,
        Some(
            mpack_growable_writer_flush
                as unsafe extern "C" fn(
                    *mut mpack_writer_t,
                    *const ::core::ffi::c_char,
                    size_t,
                ) -> (),
        ),
    );
    mpack_writer_set_teardown(
        writer,
        Some(mpack_growable_writer_teardown as unsafe extern "C" fn(*mut mpack_writer_t) -> ()),
    );
}
unsafe extern "C" fn mpack_file_writer_flush(
    mut writer: *mut mpack_writer_t,
    mut buffer: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    let mut file: *mut FILE = (*writer).context as *mut FILE;
    let mut written: size_t = test_fwrite(
        buffer as *const ::core::ffi::c_void,
        1 as size_t,
        count,
        file,
    );
    if written != count {
        mpack_writer_flag_error(writer, mpack_error_io);
    }
}
unsafe extern "C" fn mpack_file_writer_teardown(mut writer: *mut mpack_writer_t) {
    test_free((*writer).buffer as *mut ::core::ffi::c_void);
    (*writer).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*writer).context = NULL;
}
unsafe extern "C" fn mpack_file_writer_teardown_close(mut writer: *mut mpack_writer_t) {
    let mut file: *mut FILE = (*writer).context as *mut FILE;
    if !file.is_null() {
        let mut ret: ::core::ffi::c_int = test_fclose(file);
        if ret != 0 as ::core::ffi::c_int {
            mpack_writer_flag_error(writer, mpack_error_io);
        }
    }
    mpack_file_writer_teardown(writer);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_init_stdfile(
    mut writer: *mut mpack_writer_t,
    mut file: *mut FILE,
    mut close_when_done: bool,
) {
    if file.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:330\n%s\nfile is NULL\0"
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
        mpack_writer_init_error(writer, mpack_error_memory);
        if close_when_done {
            test_fclose(file);
        }
        return;
    }
    mpack_writer_init(writer, buffer, capacity);
    mpack_writer_set_context(writer, file as *mut ::core::ffi::c_void);
    mpack_writer_set_flush(
        writer,
        Some(
            mpack_file_writer_flush
                as unsafe extern "C" fn(
                    *mut mpack_writer_t,
                    *const ::core::ffi::c_char,
                    size_t,
                ) -> (),
        ),
    );
    mpack_writer_set_teardown(
        writer,
        if close_when_done as ::core::ffi::c_int != 0 {
            Some(
                mpack_file_writer_teardown_close as unsafe extern "C" fn(*mut mpack_writer_t) -> (),
            )
        } else {
            Some(mpack_file_writer_teardown as unsafe extern "C" fn(*mut mpack_writer_t) -> ())
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_init_filename(
    mut writer: *mut mpack_writer_t,
    mut filename: *const ::core::ffi::c_char,
) {
    if filename.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:351\n%s\nfilename is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"filename != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut file: *mut FILE =
        test_fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
    if file.is_null() {
        mpack_writer_init_error(writer, mpack_error_io);
        return;
    }
    mpack_writer_init_stdfile(writer, file, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_flag_error(
    mut writer: *mut mpack_writer_t,
    mut error: mpack_error_t,
) {
    if (*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*writer).error = error;
        if (*writer).error_fn.is_some() {
            (*writer).error_fn.expect("non-null function pointer")(writer, (*writer).error);
        }
    }
}
#[inline]
unsafe extern "C" fn mpack_writer_flush_unchecked(mut writer: *mut mpack_writer_t) {
    let mut used: size_t = mpack_writer_buffer_used(writer);
    (*writer).position = (*writer).buffer;
    (*writer).flush.expect("non-null function pointer")(writer, (*writer).buffer, used);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_flush_message(mut writer: *mut mpack_writer_t) {
    if (*writer).error as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    mpack_writer_flag_if_error(writer, mpack_track_check_empty(&raw mut (*writer).track));
    if (*writer).error as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if !(*writer).builder.current_build.is_null() {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:395\ncannot call mpack_writer_flush_message() while there are elements open!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    if (*writer).flush.is_none() {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:402\ncannot call mpack_writer_flush_message() without a flush function!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    if mpack_writer_buffer_used(writer) > 0 as size_t {
        mpack_writer_flush_unchecked(writer);
    }
}
#[inline(never)]
unsafe extern "C" fn mpack_writer_ensure(
    mut writer: *mut mpack_writer_t,
    mut count: size_t,
) -> bool {
    if !(count != 0 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:415\n%s\ncannot ensure zero bytes!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count != 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(count <= 32 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:418\n%s\ncannot ensure %i bytes, this is more than the minimum buffer size %i!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count <= 32\0" as *const u8 as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(count > mpack_writer_buffer_left(writer)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:421\n%s\nrequest to ensure %i bytes but there are already %i left in the buffer!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count > mpack_writer_buffer_left(writer)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            mpack_writer_buffer_left(writer) as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if !(*writer).builder.current_build.is_null() {
        mpack_builder_flush(writer);
        return mpack_writer_error(writer) as ::core::ffi::c_uint
            == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if (*writer).flush.is_none() {
        mpack_writer_flag_error(writer, mpack_error_too_big);
        return false_0 != 0;
    }
    mpack_writer_flush_unchecked(writer);
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if mpack_writer_buffer_left(writer) >= count {
        return true_0 != 0;
    }
    mpack_writer_flag_error(writer, mpack_error_io);
    return false_0 != 0;
}
#[inline(never)]
unsafe extern "C" fn mpack_write_native_straddle(
    mut writer: *mut mpack_writer_t,
    mut p: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !p.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:458\n%s\ndata pointer for %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || p != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if !(count > mpack_writer_buffer_left(writer)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:467\n%s\nbig write requested for %i bytes, but there is %i available space in buffer. should have called mpack_write_native() instead\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count > mpack_writer_buffer_left(writer)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            mpack_writer_buffer_left(writer) as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(*writer).builder.current_build.is_null() {
        loop {
            let mut step: size_t =
                (*writer).end.offset_from((*writer).position) as ::core::ffi::c_long as size_t;
            if step > count {
                step = count;
            }
            memcpy(
                (*writer).position as *mut ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                step,
            );
            (*writer).position = (*writer).position.offset(step as isize);
            p = p.offset(step as isize);
            count = count.wrapping_sub(step);
            if count == 0 as size_t {
                return;
            }
            mpack_builder_flush(writer);
            if mpack_writer_error(writer) as ::core::ffi::c_uint
                != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return;
            }
            if !((*writer).position != (*writer).end) {
                mpack_assert_fail_format(
                    b"mpack assertion failed at src/mpack/mpack-writer.c:488\n%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"writer->position != writer->end\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            } else {
            };
        }
    }
    if (*writer).flush.is_none() {
        mpack_writer_flag_error(writer, mpack_error_too_big);
        return;
    }
    mpack_writer_flush_unchecked(writer);
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if count > mpack_writer_buffer_left(writer) {
        (*writer).flush.expect("non-null function pointer")(writer, p, count);
        if mpack_writer_error(writer) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
    } else {
        memcpy(
            (*writer).position as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            count,
        );
        (*writer).position = (*writer).position.offset(count as isize);
    };
}
#[inline]
unsafe extern "C" fn mpack_write_native(
    mut writer: *mut mpack_writer_t,
    mut p: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !p.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:521\n%s\ndata pointer for %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || p != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if mpack_writer_buffer_left(writer) < count {
        mpack_write_native_straddle(writer, p, count);
    } else {
        memcpy(
            (*writer).position as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            count,
        );
        (*writer).position = (*writer).position.offset(count as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_writer_destroy(mut writer: *mut mpack_writer_t) -> mpack_error_t {
    mpack_track_destroy(
        &raw mut (*writer).track,
        (*writer).error as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if !(*builder).current_build.is_null() {
        if mpack_writer_error(writer) as ::core::ffi::c_uint
            == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            mpack_break_hit_format(
                b"mpack breakpoint hit at src/mpack/mpack-writer.c:548\nwriter cannot be destroyed with an incomplete builder unless an error was flagged!\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            mpack_writer_flag_error(writer, mpack_error_bug);
        }
        let mut page: *mut mpack_builder_page_t = (*builder).pages;
        while !page.is_null() {
            let mut next: *mut mpack_builder_page_t = (*page).next as *mut mpack_builder_page_t;
            test_free(page as *mut ::core::ffi::c_void);
            page = next;
        }
        (*writer).buffer = (*builder).stash_buffer;
        (*writer).position = (*builder).stash_position;
        (*writer).end = (*builder).stash_end;
    }
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && mpack_writer_buffer_used(writer) != 0 as size_t
        && (*writer).flush.is_some()
    {
        (*writer).flush.expect("non-null function pointer")(
            writer,
            (*writer).buffer,
            mpack_writer_buffer_used(writer),
        );
        (*writer).flush = None;
    }
    if (*writer).teardown.is_some() {
        (*writer).teardown.expect("non-null function pointer")(writer);
        (*writer).teardown = None;
    }
    return (*writer).error;
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_tag(mut writer: *mut mpack_writer_t, mut value: mpack_tag_t) {
    match value.type_0 as ::core::ffi::c_uint {
        0 => {
            mpack_break_hit_format(
                b"mpack breakpoint hit at src/mpack/mpack-writer.c:595\ncannot write a missing value!\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            mpack_writer_flag_error(writer, mpack_error_bug);
            return;
        }
        1 => {
            mpack_write_nil(writer);
            return;
        }
        2 => {
            mpack_write_bool(writer, value.v.b);
            return;
        }
        3 => {
            mpack_write_int(writer, value.v.i);
            return;
        }
        4 => {
            mpack_write_uint(writer, value.v.u);
            return;
        }
        5 => {
            mpack_write_float(writer, value.v.f);
            return;
        }
        6 => {
            mpack_write_double(writer, value.v.d);
            return;
        }
        7 => {
            mpack_start_str(writer, value.v.l);
            return;
        }
        8 => {
            mpack_start_bin(writer, value.v.l);
            return;
        }
        11 => {
            mpack_start_ext(
                writer,
                mpack_tag_ext_exttype(&raw mut value),
                mpack_tag_ext_length(&raw mut value),
            );
            return;
        }
        9 => {
            mpack_start_array(writer, value.v.n);
            return;
        }
        10 => {
            mpack_start_map(writer, value.v.n);
            return;
        }
        _ => {}
    }
    mpack_break_hit_format(
        b"mpack breakpoint hit at src/mpack/mpack-writer.c:634\nunrecognized type %i\0" as *const u8
            as *const ::core::ffi::c_char,
        value.type_0 as ::core::ffi::c_int,
    );
    mpack_writer_flag_error(writer, mpack_error_bug);
}
#[inline]
unsafe extern "C" fn mpack_write_byte_element(
    mut writer: *mut mpack_writer_t,
    mut value: ::core::ffi::c_char,
) {
    mpack_writer_track_element(writer);
    if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
    {
        let fresh0 = (*writer).position;
        (*writer).position = (*writer).position.offset(1);
        *fresh0 = value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_nil(mut writer: *mut mpack_writer_t) {
    mpack_write_byte_element(writer, 0xc0 as ::core::ffi::c_int as ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_bool(mut writer: *mut mpack_writer_t, mut value: bool) {
    mpack_write_byte_element(
        writer,
        (0xc2 as ::core::ffi::c_int
            | (if value as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })) as ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_true(mut writer: *mut mpack_writer_t) {
    mpack_write_byte_element(writer, 0xc3 as ::core::ffi::c_int as ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_false(mut writer: *mut mpack_writer_t) {
    mpack_write_byte_element(writer, 0xc2 as ::core::ffi::c_int as ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_object_bytes(
    mut writer: *mut mpack_writer_t,
    mut data: *const ::core::ffi::c_char,
    mut bytes: size_t,
) {
    mpack_writer_track_element(writer);
    mpack_write_native(writer, data, bytes);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixuint(mut p: *mut ::core::ffi::c_char, mut value: uint8_t) {
    if !(value as ::core::ffi::c_int <= 127 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:670\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value <= 127\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, value);
}
#[inline]
unsafe extern "C" fn mpack_encode_u8(mut p: *mut ::core::ffi::c_char, mut value: uint8_t) {
    if !(value as ::core::ffi::c_int > 127 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:675\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value > 127\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xcc as uint8_t);
    mpack_store_u8(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_u16(mut p: *mut ::core::ffi::c_char, mut value: uint16_t) {
    if !(value as ::core::ffi::c_int > 255 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:681\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value > (255)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xcd as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_u32(mut p: *mut ::core::ffi::c_char, mut value: uint32_t) {
    if !(value > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:687\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xce as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_u64(mut p: *mut ::core::ffi::c_char, mut value: uint64_t) {
    if !(value > 4294967295 as uint64_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:693\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value > (4294967295U)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xcf as uint8_t);
    mpack_store_u64(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixint(mut p: *mut ::core::ffi::c_char, mut value: int8_t) {
    if !(value as ::core::ffi::c_int >= -(32 as ::core::ffi::c_int)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:700\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value >= -32\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_i8(p, value);
}
#[inline]
unsafe extern "C" fn mpack_encode_i8(mut p: *mut ::core::ffi::c_char, mut value: int8_t) {
    if !((value as ::core::ffi::c_int) < -(32 as ::core::ffi::c_int)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:705\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value < -32\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xd0 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_i16(mut p: *mut ::core::ffi::c_char, mut value: int16_t) {
    if !((value as ::core::ffi::c_int) < -(128 as ::core::ffi::c_int)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:711\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value < (-128)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xd1 as uint8_t);
    mpack_store_i16(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_i32(mut p: *mut ::core::ffi::c_char, mut value: int32_t) {
    if !(value < -(32767 as int32_t) - 1 as int32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:717\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value < (-32767-1)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xd2 as uint8_t);
    mpack_store_i32(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_i64(mut p: *mut ::core::ffi::c_char, mut value: int64_t) {
    if !(value < (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as int64_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:723\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"value < (-2147483647-1)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xd3 as uint8_t);
    mpack_store_i64(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_float(
    mut p: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_float,
) {
    mpack_store_u8(p, 0xca as uint8_t);
    mpack_store_float(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_double(
    mut p: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) {
    mpack_store_u8(p, 0xcb as uint8_t);
    mpack_store_double(p.offset(1 as ::core::ffi::c_int as isize), value);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixarray(mut p: *mut ::core::ffi::c_char, mut count: uint8_t) {
    if !(count as ::core::ffi::c_int <= 15 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:753\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count <= 15\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(
        p,
        (0x90 as ::core::ffi::c_int | count as ::core::ffi::c_int) as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn mpack_encode_array16(mut p: *mut ::core::ffi::c_char, mut count: uint16_t) {
    if !(count as ::core::ffi::c_int > 15 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:758\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > 15\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xdc as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_array32(mut p: *mut ::core::ffi::c_char, mut count: uint32_t) {
    if !(count > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:764\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xdd as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixmap(mut p: *mut ::core::ffi::c_char, mut count: uint8_t) {
    if !(count as ::core::ffi::c_int <= 15 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:770\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count <= 15\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(
        p,
        (0x80 as ::core::ffi::c_int | count as ::core::ffi::c_int) as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn mpack_encode_map16(mut p: *mut ::core::ffi::c_char, mut count: uint16_t) {
    if !(count as ::core::ffi::c_int > 15 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:775\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > 15\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xde as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_map32(mut p: *mut ::core::ffi::c_char, mut count: uint32_t) {
    if !(count > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:781\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xdf as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixstr(mut p: *mut ::core::ffi::c_char, mut count: uint8_t) {
    if !(count as ::core::ffi::c_int <= 31 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:787\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count <= 31\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(
        p,
        (0xa0 as ::core::ffi::c_int | count as ::core::ffi::c_int) as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn mpack_encode_str8(mut p: *mut ::core::ffi::c_char, mut count: uint8_t) {
    if !(count as ::core::ffi::c_int > 31 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:792\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > 31\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xd9 as uint8_t);
    mpack_store_u8(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_str16(mut p: *mut ::core::ffi::c_char, mut count: uint16_t) {
    if !(count as ::core::ffi::c_int > 31 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:800\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > 31\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xda as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_str32(mut p: *mut ::core::ffi::c_char, mut count: uint32_t) {
    if !(count > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:806\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xdb as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_bin8(mut p: *mut ::core::ffi::c_char, mut count: uint8_t) {
    mpack_store_u8(p, 0xc4 as uint8_t);
    mpack_store_u8(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_bin16(mut p: *mut ::core::ffi::c_char, mut count: uint16_t) {
    if !(count as ::core::ffi::c_int > 255 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:817\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (255)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xc5 as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_bin32(mut p: *mut ::core::ffi::c_char, mut count: uint32_t) {
    if !(count > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:823\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xc6 as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), count);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixext1(mut p: *mut ::core::ffi::c_char, mut exttype: int8_t) {
    mpack_store_u8(p, 0xd4 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixext2(mut p: *mut ::core::ffi::c_char, mut exttype: int8_t) {
    mpack_store_u8(p, 0xd5 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixext4(mut p: *mut ::core::ffi::c_char, mut exttype: int8_t) {
    mpack_store_u8(p, 0xd6 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixext8(mut p: *mut ::core::ffi::c_char, mut exttype: int8_t) {
    mpack_store_u8(p, 0xd7 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_fixext16(mut p: *mut ::core::ffi::c_char, mut exttype: int8_t) {
    mpack_store_u8(p, 0xd8 as uint8_t);
    mpack_store_i8(p.offset(1 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_ext8(
    mut p: *mut ::core::ffi::c_char,
    mut exttype: int8_t,
    mut count: uint8_t,
) {
    if !(count as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && count as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        && count as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        && count as ::core::ffi::c_int != 8 as ::core::ffi::c_int
        && count as ::core::ffi::c_int != 16 as ::core::ffi::c_int)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:855\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count != 1 && count != 2 && count != 4 && count != 8 && count != 16\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xc7 as uint8_t);
    mpack_store_u8(p.offset(1 as ::core::ffi::c_int as isize), count);
    mpack_store_i8(p.offset(2 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_ext16(
    mut p: *mut ::core::ffi::c_char,
    mut exttype: int8_t,
    mut count: uint16_t,
) {
    if !(count as ::core::ffi::c_int > 255 as ::core::ffi::c_int) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:862\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (255)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xc8 as uint8_t);
    mpack_store_u16(p.offset(1 as ::core::ffi::c_int as isize), count);
    mpack_store_i8(p.offset(3 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_ext32(
    mut p: *mut ::core::ffi::c_char,
    mut exttype: int8_t,
    mut count: uint32_t,
) {
    if !(count > 65535 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:869\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"count > (65535)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_store_u8(p, 0xc9 as uint8_t);
    mpack_store_u32(p.offset(1 as ::core::ffi::c_int as isize), count);
    mpack_store_i8(p.offset(5 as ::core::ffi::c_int as isize), exttype);
}
#[inline]
unsafe extern "C" fn mpack_encode_timestamp_4(
    mut p: *mut ::core::ffi::c_char,
    mut seconds: uint32_t,
) {
    mpack_encode_fixext4(p, MPACK_EXTTYPE_TIMESTAMP);
    mpack_store_u32(p.offset(MPACK_TAG_SIZE_FIXEXT4 as isize), seconds);
}
#[inline]
unsafe extern "C" fn mpack_encode_timestamp_8(
    mut p: *mut ::core::ffi::c_char,
    mut seconds: int64_t,
    mut nanoseconds: uint32_t,
) {
    if !(nanoseconds <= 999999999 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:881\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nanoseconds <= 999999999\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_encode_fixext8(p, MPACK_EXTTYPE_TIMESTAMP);
    let mut encoded: uint64_t =
        (nanoseconds as uint64_t) << 34 as ::core::ffi::c_int | seconds as uint64_t;
    mpack_store_u64(p.offset(MPACK_TAG_SIZE_FIXEXT8 as isize), encoded);
}
#[inline]
unsafe extern "C" fn mpack_encode_timestamp_12(
    mut p: *mut ::core::ffi::c_char,
    mut seconds: int64_t,
    mut nanoseconds: uint32_t,
) {
    if !(nanoseconds <= 999999999 as uint32_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:888\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nanoseconds <= 999999999\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_encode_ext8(p, MPACK_EXTTYPE_TIMESTAMP, 12 as uint8_t);
    mpack_store_u32(p.offset(MPACK_TAG_SIZE_EXT8 as isize), nanoseconds);
    mpack_store_i64(
        p.offset(MPACK_TAG_SIZE_EXT8 as isize)
            .offset(4 as ::core::ffi::c_int as isize),
        seconds,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_u8(mut writer: *mut mpack_writer_t, mut value: uint8_t) {
    mpack_writer_track_element(writer);
    if value as ::core::ffi::c_int <= 127 as ::core::ffi::c_int {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixuint((*writer).position, value);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_u8((*writer).position, value);
        (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_u16(mut writer: *mut mpack_writer_t, mut value: uint16_t) {
    mpack_writer_track_element(writer);
    if value as ::core::ffi::c_int <= 127 as ::core::ffi::c_int {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixuint((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if value as ::core::ffi::c_int <= MPACK_UINT8_MAX {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u8((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_u16((*writer).position, value);
        (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_u32(mut writer: *mut mpack_writer_t, mut value: uint32_t) {
    mpack_writer_track_element(writer);
    if value <= 127 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixuint((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if value <= MPACK_UINT8_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u8((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if value <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u16((*writer).position, value as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_u32((*writer).position, value);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_u64(mut writer: *mut mpack_writer_t, mut value: uint64_t) {
    mpack_writer_track_element(writer);
    if value <= 127 as uint64_t {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixuint((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if value <= MPACK_UINT8_MAX as uint64_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u8((*writer).position, value as uint8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if value <= MPACK_UINT16_MAX as uint64_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u16((*writer).position, value as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if value <= MPACK_UINT32_MAX as uint64_t {
        if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u32((*writer).position, value as uint32_t);
            (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 9 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 9 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_u64((*writer).position, value);
        (*writer).position = (*writer).position.offset(9 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_i8(mut writer: *mut mpack_writer_t, mut value: int8_t) {
    mpack_writer_track_element(writer);
    if value as ::core::ffi::c_int >= -(32 as ::core::ffi::c_int) {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixint((*writer).position, value);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_i8((*writer).position, value);
        (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_i16(mut writer: *mut mpack_writer_t, mut value: int16_t) {
    mpack_writer_track_element(writer);
    if value as ::core::ffi::c_int >= -(32 as ::core::ffi::c_int) {
        if value as ::core::ffi::c_int <= 127 as ::core::ffi::c_int {
            if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_fixint((*writer).position, value as int8_t);
                (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
            }
        } else if value as ::core::ffi::c_int <= MPACK_UINT8_MAX {
            if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u8((*writer).position, value as uint8_t);
                (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
            }
        } else if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u16((*writer).position, value as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if value as ::core::ffi::c_int >= MPACK_INT8_MIN {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i8((*writer).position, value as int8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_i16((*writer).position, value);
        (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_i32(mut writer: *mut mpack_writer_t, mut value: int32_t) {
    mpack_writer_track_element(writer);
    if value >= -(32 as int32_t) {
        if value <= 127 as int32_t {
            if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_fixint((*writer).position, value as int8_t);
                (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
            }
        } else if value <= MPACK_UINT8_MAX as int32_t {
            if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u8((*writer).position, value as uint8_t);
                (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
            }
        } else if value <= MPACK_UINT16_MAX as int32_t {
            if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u16((*writer).position, value as uint16_t);
                (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
            }
        } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u32((*writer).position, value as uint32_t);
            (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
        }
    } else if value >= MPACK_INT8_MIN as int32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i8((*writer).position, value as int8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if value >= MPACK_INT16_MIN as int32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i16((*writer).position, value as int16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_i32((*writer).position, value);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_i64(mut writer: *mut mpack_writer_t, mut value: int64_t) {
    mpack_writer_track_element(writer);
    if value >= -(32 as ::core::ffi::c_int) as int64_t {
        if value <= 127 as int64_t {
            if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_fixint((*writer).position, value as int8_t);
                (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
            }
        } else if value <= MPACK_UINT8_MAX as int64_t {
            if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u8((*writer).position, value as uint8_t);
                (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
            }
        } else if value <= MPACK_UINT16_MAX as int64_t {
            if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u16((*writer).position, value as uint16_t);
                (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
            }
        } else if value <= MPACK_UINT32_MAX as int64_t {
            if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_u32((*writer).position, value as uint32_t);
                (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
            }
        } else if (mpack_writer_buffer_left(writer) >= 9 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 9 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_u64((*writer).position, value as uint64_t);
            (*writer).position = (*writer).position.offset(9 as ::core::ffi::c_int as isize);
        }
    } else if value >= MPACK_INT8_MIN as int64_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i8((*writer).position, value as int8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if value >= MPACK_INT16_MIN as int64_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i16((*writer).position, value as int16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if value >= MPACK_INT32_MIN as int64_t {
        if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_i32((*writer).position, value as int32_t);
            (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 9 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 9 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_i64((*writer).position, value);
        (*writer).position = (*writer).position.offset(9 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_float(
    mut writer: *mut mpack_writer_t,
    mut value: ::core::ffi::c_float,
) {
    mpack_writer_track_element(writer);
    if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_float((*writer).position, value);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_double(
    mut writer: *mut mpack_writer_t,
    mut value: ::core::ffi::c_double,
) {
    mpack_writer_track_element(writer);
    if (mpack_writer_buffer_left(writer) >= 9 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 9 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_double((*writer).position, value);
        (*writer).position = (*writer).position.offset(9 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_timestamp(
    mut writer: *mut mpack_writer_t,
    mut seconds: int64_t,
    mut nanoseconds: uint32_t,
) {
    if (*writer).version as ::core::ffi::c_uint
        <= mpack_version_v4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1099\nTimestamps require spec version v5 or later. This writer is in v%i mode.\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*writer).version as ::core::ffi::c_int,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    if nanoseconds > MPACK_TIMESTAMP_NANOSECONDS_MAX as uint32_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1106\ntimestamp nanoseconds out of bounds: %u\0"
                as *const u8 as *const ::core::ffi::c_char,
            nanoseconds,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    mpack_writer_track_element(writer);
    if seconds < 0 as int64_t || seconds >= (1 as int64_t) << 34 as ::core::ffi::c_int {
        if (mpack_writer_buffer_left(writer)
            >= (3 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(
                writer,
                (3 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as size_t,
            ) as ::core::ffi::c_int
                != 0
        {
            mpack_encode_timestamp_12((*writer).position, seconds, nanoseconds);
            (*writer).position = (*writer)
                .position
                .offset((3 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize);
        }
    } else if seconds > MPACK_UINT32_MAX as int64_t || nanoseconds > 0 as uint32_t {
        if (mpack_writer_buffer_left(writer)
            >= (2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(
                writer,
                (2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as size_t,
            ) as ::core::ffi::c_int
                != 0
        {
            mpack_encode_timestamp_8((*writer).position, seconds, nanoseconds);
            (*writer).position = (*writer)
                .position
                .offset((2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize);
        }
    } else if (mpack_writer_buffer_left(writer)
        >= (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(
            writer,
            (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t,
        ) as ::core::ffi::c_int
            != 0
    {
        mpack_encode_timestamp_4((*writer).position, seconds as uint32_t);
        (*writer).position = (*writer)
            .position
            .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize);
    }
}
unsafe extern "C" fn mpack_write_array_notrack(
    mut writer: *mut mpack_writer_t,
    mut count: uint32_t,
) {
    if count <= 15 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixarray((*writer).position, count as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_array16((*writer).position, count as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_array32((*writer).position, count);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn mpack_write_map_notrack(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    if count <= 15 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixmap((*writer).position, count as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_map16((*writer).position, count as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_map32((*writer).position, count);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_start_array(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    mpack_writer_track_element(writer);
    mpack_write_array_notrack(writer, count);
    mpack_writer_track_push(writer, mpack_type_array, count);
    mpack_builder_compound_push(writer);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_start_map(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    mpack_writer_track_element(writer);
    mpack_write_map_notrack(writer, count);
    mpack_writer_track_push(writer, mpack_type_map, count);
    mpack_builder_compound_push(writer);
}
unsafe extern "C" fn mpack_start_str_notrack(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    if count <= 31 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 1 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 1 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixstr((*writer).position, count as uint8_t);
            (*writer).position = (*writer).position.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT8_MAX as uint32_t
        && (*writer).version as ::core::ffi::c_uint
            >= mpack_version_v5 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_str8((*writer).position, count as uint8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_str16((*writer).position, count as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_str32((*writer).position, count);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn mpack_start_bin_notrack(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    if (*writer).version as ::core::ffi::c_uint
        <= mpack_version_v4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_start_str_notrack(writer, count);
        return;
    }
    if count <= MPACK_UINT8_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_bin8((*writer).position, count as uint8_t);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_bin16((*writer).position, count as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_bin32((*writer).position, count);
        (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpack_start_str(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    mpack_writer_track_element(writer);
    mpack_start_str_notrack(writer, count);
    mpack_writer_track_push(writer, mpack_type_str, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_start_bin(mut writer: *mut mpack_writer_t, mut count: uint32_t) {
    mpack_writer_track_element(writer);
    mpack_start_bin_notrack(writer, count);
    mpack_writer_track_push(writer, mpack_type_bin, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_start_ext(
    mut writer: *mut mpack_writer_t,
    mut exttype: int8_t,
    mut count: uint32_t,
) {
    if (*writer).version as ::core::ffi::c_uint
        <= mpack_version_v4 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1212\nExt types require spec version v5 or later. This writer is in v%i mode.\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*writer).version as ::core::ffi::c_int,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    mpack_writer_track_element(writer);
    if count == 1 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixext1((*writer).position, exttype);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count == 2 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixext2((*writer).position, exttype);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count == 4 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixext4((*writer).position, exttype);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count == 8 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixext8((*writer).position, exttype);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count == 16 as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_fixext16((*writer).position, exttype);
            (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT8_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_ext8((*writer).position, exttype, count as uint8_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
    } else if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 4 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 4 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_ext16((*writer).position, exttype, count as uint16_t);
            (*writer).position = (*writer).position.offset(4 as ::core::ffi::c_int as isize);
        }
    } else if (mpack_writer_buffer_left(writer) >= 6 as size_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
        || mpack_writer_ensure(writer, 6 as size_t) as ::core::ffi::c_int != 0
    {
        mpack_encode_ext32((*writer).position, exttype, count);
        (*writer).position = (*writer).position.offset(6 as ::core::ffi::c_int as isize);
    }
    mpack_writer_track_push(writer, mpack_type_ext, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_str(
    mut writer: *mut mpack_writer_t,
    mut data: *const ::core::ffi::c_char,
    mut count: uint32_t,
) {
    if !(count == 0 as uint32_t || !data.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1249\n%s\ndata for string of length %i is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || data != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_writer_track_element(writer);
    if count <= 31 as uint32_t {
        let mut size: size_t = count.wrapping_add(MPACK_TAG_SIZE_FIXSTR as uint32_t) as size_t;
        if (mpack_writer_buffer_left(writer) >= size) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, size) as ::core::ffi::c_int != 0
        {
            let mut p: *mut ::core::ffi::c_char = (*writer).position;
            mpack_encode_fixstr(p, count as uint8_t);
            memcpy(
                p.offset(MPACK_TAG_SIZE_FIXSTR as isize) as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                count as size_t,
            );
            (*writer).position = (*writer)
                .position
                .offset(count.wrapping_add(MPACK_TAG_SIZE_FIXSTR as uint32_t) as isize);
        }
        return;
    }
    if count <= MPACK_UINT8_MAX as uint32_t
        && (*writer).version as ::core::ffi::c_uint
            >= mpack_version_v5 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if count.wrapping_add(MPACK_TAG_SIZE_STR8 as uint32_t) as size_t
            <= mpack_writer_buffer_left(writer)
        {
            let mut p_0: *mut ::core::ffi::c_char = (*writer).position;
            mpack_encode_str8(p_0, count as uint8_t);
            memcpy(
                p_0.offset(MPACK_TAG_SIZE_STR8 as isize) as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                count as size_t,
            );
            (*writer).position = (*writer)
                .position
                .offset(count.wrapping_add(MPACK_TAG_SIZE_STR8 as uint32_t) as isize);
        } else {
            if (mpack_writer_buffer_left(writer) >= 2 as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
                || mpack_writer_ensure(writer, 2 as size_t) as ::core::ffi::c_int != 0
            {
                mpack_encode_str8((*writer).position, count as uint8_t);
                (*writer).position = (*writer).position.offset(2 as ::core::ffi::c_int as isize);
            }
            mpack_write_native(writer, data, count as size_t);
        }
        return;
    }
    if count <= MPACK_UINT16_MAX as uint32_t {
        if (mpack_writer_buffer_left(writer) >= 3 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 3 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_str16((*writer).position, count as uint16_t);
            (*writer).position = (*writer).position.offset(3 as ::core::ffi::c_int as isize);
        }
        mpack_write_native(writer, data, count as size_t);
    } else {
        if (mpack_writer_buffer_left(writer) >= 5 as size_t) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
            || mpack_writer_ensure(writer, 5 as size_t) as ::core::ffi::c_int != 0
        {
            mpack_encode_str32((*writer).position, count);
            (*writer).position = (*writer).position.offset(5 as ::core::ffi::c_int as isize);
        }
        mpack_write_native(writer, data, count as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_bin(
    mut writer: *mut mpack_writer_t,
    mut data: *const ::core::ffi::c_char,
    mut count: uint32_t,
) {
    if !(count == 0 as uint32_t || !data.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1304\n%s\ndata pointer for bin of %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || data != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_start_bin(writer, count);
    mpack_write_bytes(writer, data, count as size_t);
    mpack_finish_bin(writer);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_ext(
    mut writer: *mut mpack_writer_t,
    mut exttype: int8_t,
    mut data: *const ::core::ffi::c_char,
    mut count: uint32_t,
) {
    if !(count == 0 as uint32_t || !data.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1312\n%s\ndata pointer for ext of type %i and %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || data != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            exttype as ::core::ffi::c_int,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_start_ext(writer, exttype, count);
    mpack_write_bytes(writer, data, count as size_t);
    mpack_finish_ext(writer);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_bytes(
    mut writer: *mut mpack_writer_t,
    mut data: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if !(count == 0 as size_t || !data.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1320\n%s\ndata pointer for %i bytes is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"count == 0 || data != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            count as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_writer_track_bytes(writer, count);
    mpack_write_native(writer, data, count);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_cstr(
    mut writer: *mut mpack_writer_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1326\n%s\ncstr pointer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = test_strlen(cstr);
    if length > MPACK_UINT32_MAX as size_t {
        mpack_writer_flag_error(writer, mpack_error_invalid);
    }
    mpack_write_str(writer, cstr, length as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_cstr_or_nil(
    mut writer: *mut mpack_writer_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    if !cstr.is_null() {
        mpack_write_cstr(writer, cstr);
    } else {
        mpack_write_nil(writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_utf8(
    mut writer: *mut mpack_writer_t,
    mut str: *const ::core::ffi::c_char,
    mut length: uint32_t,
) {
    if !(length == 0 as uint32_t || !str.is_null()) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1341\n%s\ndata for string of length %i is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"length == 0 || str != ((void*)0)\0" as *const u8
                as *const ::core::ffi::c_char,
            length as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !mpack_utf8_check(str, length as size_t) {
        mpack_writer_flag_error(writer, mpack_error_invalid);
        return;
    }
    mpack_write_str(writer, str, length);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_utf8_cstr(
    mut writer: *mut mpack_writer_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    if cstr.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1350\n%s\ncstr pointer is NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"cstr != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut length: size_t = test_strlen(cstr);
    if length > MPACK_UINT32_MAX as size_t {
        mpack_writer_flag_error(writer, mpack_error_invalid);
        return;
    }
    mpack_write_utf8(writer, cstr, length as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_write_utf8_cstr_or_nil(
    mut writer: *mut mpack_writer_t,
    mut cstr: *const ::core::ffi::c_char,
) {
    if !cstr.is_null() {
        mpack_write_utf8_cstr(writer, cstr);
    } else {
        mpack_write_nil(writer);
    };
}
pub const MPACK_BUILD_ALIGNMENT: usize = ::core::mem::align_of::<mpack_build_t>();
#[inline]
unsafe extern "C" fn mpack_builder_check_sizes(mut writer: *mut mpack_writer_t) {
    if MPACK_BUILDER_PAGE_SIZE
        < (::core::mem::size_of::<mpack_builder_page_t>() as usize)
            .wrapping_add(::core::mem::size_of::<mpack_build_t>() as usize)
            .wrapping_add(MPACK_WRITER_MINIMUM_BUFFER_SIZE as usize)
    {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1437\nMPACK_BUILDER_PAGE_SIZE is too small to be useful!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
    }
}
#[inline]
unsafe extern "C" fn mpack_builder_page_size(
    mut writer: *mut mpack_writer_t,
    mut page: *mut mpack_builder_page_t,
) -> size_t {
    return MPACK_BUILDER_PAGE_SIZE;
}
#[inline]
unsafe extern "C" fn mpack_builder_align_build(mut bytes_used: size_t) -> size_t {
    let mut offset: size_t = bytes_used;
    offset = (offset as ::core::ffi::c_ulong)
        .wrapping_add(MPACK_BUILD_ALIGNMENT.wrapping_sub(1 as usize) as ::core::ffi::c_ulong)
        as size_t as size_t;
    offset = (offset as ::core::ffi::c_ulong)
        .wrapping_sub(offset.wrapping_rem(MPACK_BUILD_ALIGNMENT) as ::core::ffi::c_ulong)
        as size_t as size_t;
    return offset;
}
#[inline]
unsafe extern "C" fn mpack_builder_free_page(
    mut writer: *mut mpack_writer_t,
    mut page: *mut mpack_builder_page_t,
) {
    test_free(page as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn mpack_builder_page_remaining(
    mut writer: *mut mpack_writer_t,
    mut page: *mut mpack_builder_page_t,
) -> size_t {
    return mpack_builder_page_size(writer, page).wrapping_sub((*page).bytes_used);
}
unsafe extern "C" fn mpack_builder_configure_buffer(mut writer: *mut mpack_writer_t) {
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    let mut page: *mut mpack_builder_page_t = (*builder).current_page;
    if page.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1482\n%s\npage is null??\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"page != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*writer).buffer = (page as *mut ::core::ffi::c_char).offset((*page).bytes_used as isize);
    (*writer).position = (page as *mut ::core::ffi::c_char).offset((*page).bytes_used as isize);
    (*writer).end =
        (page as *mut ::core::ffi::c_char).offset(mpack_builder_page_size(writer, page) as isize);
}
unsafe extern "C" fn mpack_builder_add_page(mut writer: *mut mpack_writer_t) {
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if !((*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1494\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"writer->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut page: *mut mpack_builder_page_t =
        test_malloc(MPACK_BUILDER_PAGE_SIZE) as *mut mpack_builder_page_t;
    if page.is_null() {
        mpack_writer_flag_error(writer, mpack_error_memory);
        return;
    }
    (*page).next = ::core::ptr::null_mut::<mpack_builder_page_t>();
    (*page).bytes_used = ::core::mem::size_of::<mpack_builder_page_t>() as usize as size_t;
    (*(*builder).current_page).next = page as *mut mpack_builder_page_t;
    (*builder).current_page = page;
}
unsafe extern "C" fn mpack_builder_apply_writes(mut writer: *mut mpack_writer_t) {
    if !((*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1513\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"writer->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    let mut bytes_written: size_t =
        (*writer).position.offset_from((*writer).buffer) as ::core::ffi::c_long as size_t;
    if (*builder).current_page.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1522\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"builder->current_page != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*builder).latest_build.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1523\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"builder->latest_build != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*(*builder).current_page).bytes_used = (*(*builder).current_page)
        .bytes_used
        .wrapping_add(bytes_written);
    (*(*builder).latest_build).bytes = (*(*builder).latest_build).bytes.wrapping_add(bytes_written);
}
unsafe extern "C" fn mpack_builder_flush(mut writer: *mut mpack_writer_t) {
    if !((*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1530\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"writer->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_builder_apply_writes(writer);
    mpack_builder_add_page(writer);
    mpack_builder_configure_buffer(writer);
}
#[inline(never)]
unsafe extern "C" fn mpack_builder_begin(mut writer: *mut mpack_writer_t) {
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if !((*writer).error as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1538\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"writer->error == mpack_ok\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(*builder).current_build.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1539\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"builder->current_build == ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(*builder).latest_build.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1540\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"builder->latest_build == ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !(*builder).pages.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1541\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"builder->pages == ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    (*builder).stash_buffer = (*writer).buffer;
    (*builder).stash_position = (*writer).position;
    (*builder).stash_end = (*writer).end;
    let mut page: *mut mpack_builder_page_t = ::core::ptr::null_mut::<mpack_builder_page_t>();
    page = test_malloc(MPACK_BUILDER_PAGE_SIZE) as *mut mpack_builder_page_t;
    if page.is_null() {
        mpack_writer_flag_error(writer, mpack_error_memory);
        return;
    }
    (*page).next = ::core::ptr::null_mut::<mpack_builder_page_t>();
    (*page).bytes_used = ::core::mem::size_of::<mpack_builder_page_t>() as usize as size_t;
    (*builder).pages = page;
    (*builder).current_page = page;
}
unsafe extern "C" fn mpack_builder_build(
    mut writer: *mut mpack_writer_t,
    mut type_0: mpack_type_t,
) {
    mpack_builder_check_sizes(writer);
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    mpack_writer_track_element(writer);
    mpack_writer_track_push_builder(writer, type_0);
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if (*builder).current_build.is_null() {
        mpack_builder_begin(writer);
    } else {
        mpack_builder_apply_writes(writer);
    }
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut offset: size_t = mpack_builder_align_build((*(*builder).current_page).bytes_used);
    if offset.wrapping_add(::core::mem::size_of::<mpack_build_t>() as size_t)
        > mpack_builder_page_size(writer, (*builder).current_page)
    {
        mpack_builder_add_page(writer);
        offset = mpack_builder_align_build((*(*builder).current_page).bytes_used);
    }
    let mut page: *mut mpack_builder_page_t = (*builder).current_page;
    (*page).bytes_used = offset.wrapping_add(::core::mem::size_of::<mpack_build_t>() as size_t);
    if !((*page).bytes_used <= mpack_builder_page_size(writer, page)) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1606\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"page->bytes_used <= mpack_builder_page_size(writer, page)\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut build: *mut mpack_build_t =
        (page as *mut ::core::ffi::c_char).offset(offset as isize) as *mut mpack_build_t;
    (*build).parent = (*builder).current_build as *mut mpack_build_t;
    (*build).bytes = 0 as size_t;
    (*build).count = 0 as uint32_t;
    (*build).type_0 = type_0;
    (*build).key_needs_value = false_0 != 0;
    (*build).nested_compound_elements = 0 as uint32_t;
    (*builder).current_build = build;
    (*builder).latest_build = build;
    if mpack_builder_page_remaining(writer, page) < MPACK_WRITER_MINIMUM_BUFFER_SIZE as size_t {
        mpack_builder_add_page(writer);
        if mpack_writer_error(writer) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
    }
    if !(mpack_builder_page_remaining(writer, (*builder).current_page) >= 32 as size_t) {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1633\n%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"mpack_builder_page_remaining(writer, builder->current_page) >= 32\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    mpack_builder_configure_buffer(writer);
}
#[inline(never)]
unsafe extern "C" fn mpack_builder_resolve(mut writer: *mut mpack_writer_t) {
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if !(mpack_writer_error(writer) as ::core::ffi::c_uint
        == mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1644\n%s\ncan't resolve in error state!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"mpack_writer_error(writer) == mpack_ok\0" as *const u8
                as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    let mut error_fn: mpack_writer_error_t = (*writer).error_fn;
    (*writer).error_fn = None;
    let mut page: *mut mpack_builder_page_t = (*builder).pages;
    (*writer).buffer = (*builder).stash_buffer;
    (*writer).position = (*builder).stash_position;
    (*writer).end = (*builder).stash_end;
    (*builder).current_build = ::core::ptr::null_mut::<mpack_build_t>();
    (*builder).latest_build = ::core::ptr::null_mut::<mpack_build_t>();
    (*builder).current_page = ::core::ptr::null_mut::<mpack_builder_page_t>();
    (*builder).pages = ::core::ptr::null_mut::<mpack_builder_page_t>();
    let mut offset: size_t =
        mpack_builder_align_build(::core::mem::size_of::<mpack_builder_page_t>() as size_t);
    let mut build: *mut mpack_build_t =
        (page as *mut ::core::ffi::c_char).offset(offset as isize) as *mut mpack_build_t;
    offset = (offset as ::core::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<mpack_build_t>() as usize as ::core::ffi::c_ulong)
        as size_t as size_t;
    loop {
        match (*build).type_0 as ::core::ffi::c_uint {
            10 => {
                mpack_write_map_notrack(writer, (*build).count);
            }
            9 => {
                mpack_write_array_notrack(writer, (*build).count);
            }
            _ => {
                mpack_break_hit_format(
                    b"mpack breakpoint hit at src/mpack/mpack-writer.c:1699\ninvalid type in builder?\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                mpack_writer_flag_error(writer, mpack_error_bug);
                return;
            }
        }
        let mut left: size_t = (*build).bytes;
        build = ::core::ptr::null_mut::<mpack_build_t>();
        while left > 0 as size_t {
            let mut bytes_used: size_t = (*page).bytes_used;
            if offset < bytes_used {
                let mut step: size_t = bytes_used.wrapping_sub(offset);
                if step > left {
                    step = left;
                }
                mpack_write_native(
                    writer,
                    (page as *mut ::core::ffi::c_char).offset(offset as isize),
                    step,
                );
                offset = offset.wrapping_add(step);
                left = left.wrapping_sub(step);
            }
            if left == 0 as size_t {
                break;
            }
            let mut next_page: *mut mpack_builder_page_t =
                (*page).next as *mut mpack_builder_page_t;
            mpack_builder_free_page(writer, page);
            page = next_page;
            offset = ::core::mem::size_of::<mpack_builder_page_t>() as usize as size_t;
        }
        offset = mpack_builder_align_build(offset);
        if offset.wrapping_add(::core::mem::size_of::<mpack_build_t>() as size_t)
            > mpack_builder_page_size(writer, page)
        {
            let mut next_page_0: *mut mpack_builder_page_t =
                (*page).next as *mut mpack_builder_page_t;
            mpack_builder_free_page(writer, page);
            page = next_page_0;
            if page.is_null() {
                break;
            }
            offset =
                mpack_builder_align_build(::core::mem::size_of::<mpack_builder_page_t>() as size_t);
        }
        if offset.wrapping_add(::core::mem::size_of::<mpack_build_t>() as size_t)
            > (*page).bytes_used
        {
            mpack_builder_free_page(writer, page);
            break;
        } else {
            build =
                (page as *mut ::core::ffi::c_char).offset(offset as isize) as *mut mpack_build_t;
            offset = (offset as ::core::ffi::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<mpack_build_t>() as usize as ::core::ffi::c_ulong
                ) as size_t as size_t;
        }
    }
    (*writer).error_fn = error_fn;
    if (*writer).error_fn.is_some()
        && mpack_writer_error(writer) as ::core::ffi::c_uint
            != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*writer).error_fn.expect("non-null function pointer")(writer, (*writer).error);
    }
}
unsafe extern "C" fn mpack_builder_complete(
    mut writer: *mut mpack_writer_t,
    mut type_0: mpack_type_t,
) {
    mpack_writer_track_pop_builder(writer, type_0);
    if mpack_writer_error(writer) as ::core::ffi::c_uint
        != mpack_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut builder: *mut mpack_builder_t = &raw mut (*writer).builder;
    if (*builder).current_build.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1778\n%s\nno build in progress!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"builder->current_build != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*builder).latest_build.is_null() {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1779\n%s\nmissing latest build!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"builder->latest_build != ((void*)0)\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if !((*(*builder).current_build).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint)
    {
        mpack_assert_fail_format(
            b"mpack assertion failed at src/mpack/mpack-writer.c:1780\n%s\ncompleting wrong type!\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"builder->current_build->type == type\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    } else {
    };
    if (*(*builder).current_build).key_needs_value {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1784\nan odd number of elements were written in a map!\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    if (*(*builder).current_build).nested_compound_elements != 0 as uint32_t {
        mpack_break_hit_format(
            b"mpack breakpoint hit at src/mpack/mpack-writer.c:1790\nthere is a nested unfinished non-build map or array in this build.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        mpack_writer_flag_error(writer, mpack_error_bug);
        return;
    }
    mpack_builder_apply_writes(writer);
    if !(*(*builder).current_build).parent.is_null() {
        (*builder).current_build = (*(*builder).current_build).parent as *mut mpack_build_t;
        mpack_builder_configure_buffer(writer);
    } else {
        mpack_builder_resolve(writer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpack_build_map(mut writer: *mut mpack_writer_t) {
    mpack_builder_build(writer, mpack_type_map);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_build_array(mut writer: *mut mpack_writer_t) {
    mpack_builder_build(writer, mpack_type_array);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_complete_map(mut writer: *mut mpack_writer_t) {
    mpack_builder_complete(writer, mpack_type_map);
}
#[no_mangle]
pub unsafe extern "C" fn mpack_complete_array(mut writer: *mut mpack_writer_t) {
    mpack_builder_complete(writer, mpack_type_array);
}
pub const MPACK_BUFFER_SIZE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MPACK_BUILDER_PAGE_SIZE: usize = (::core::mem::size_of::<mpack_builder_page_t>()
    as usize)
    .wrapping_add(::core::mem::size_of::<mpack_build_t>() as usize)
    .wrapping_add(MPACK_WRITER_MINIMUM_BUFFER_SIZE as usize)
    .wrapping_add(77 as usize);
