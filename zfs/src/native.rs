use std::os::raw::{c_char, c_int};

#[allow(non_camel_case_types)]
pub type c_bool = u32;

// nvlist

#[repr(C)]
pub struct nvlist_t {
    _unused: [u8; 0],
}

pub const NV_UNIQUE_NAME: u32 = 1;

#[allow(non_camel_case_types)]
pub type nvlist_add_func<T> = unsafe extern "C" fn(*mut nvlist_t, *const c_char, T) -> c_int;

#[link(name = "nvpair")]
extern "C" {
    pub fn nvlist_alloc(ptr: *mut *mut nvlist_t, nvflag: u32, flag: c_int) -> c_int;
    pub fn nvlist_free(ptr: *mut nvlist_t);

    pub fn nvlist_add_boolean_value(
        ptr: *mut nvlist_t,
        name: *const c_char,
        value: c_bool,
    ) -> c_int;
    pub fn nvlist_add_int8(ptr: *mut nvlist_t, name: *const c_char, value: i8) -> c_int;
    pub fn nvlist_add_uint8(ptr: *mut nvlist_t, name: *const c_char, value: u8) -> c_int;
    pub fn nvlist_add_int16(ptr: *mut nvlist_t, name: *const c_char, value: i16) -> c_int;
    pub fn nvlist_add_uint16(ptr: *mut nvlist_t, name: *const c_char, value: u16) -> c_int;
    pub fn nvlist_add_int32(ptr: *mut nvlist_t, name: *const c_char, value: i32) -> c_int;
    pub fn nvlist_add_uint32(ptr: *mut nvlist_t, name: *const c_char, value: u32) -> c_int;
    pub fn nvlist_add_int64(ptr: *mut nvlist_t, name: *const c_char, value: i64) -> c_int;
    pub fn nvlist_add_uint64(ptr: *mut nvlist_t, name: *const c_char, value: u64) -> c_int;
    pub fn nvlist_add_string(
        ptr: *mut nvlist_t,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;
}

// zfs

#[repr(C)]
pub struct libzfs_handle_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct zfs_handle_t {
    _unused: [u8; 0],
}

pub mod zfs_type_t {
    #![allow(unused)]
    pub type Value = u32;

    pub const ZFS_TYPE_FILESYSTEM: Value = 1;
    pub const ZFS_TYPE_SNAPSHOT: Value = 2;
    pub const ZFS_TYPE_VOLUME: Value = 4;
    pub const ZFS_TYPE_POOL: Value = 8;
    pub const ZFS_TYPE_BOOKMARK: Value = 16;
}

#[link(name = "zfs")]
extern "C" {
    pub fn libzfs_init() -> *mut libzfs_handle_t;
    pub fn libzfs_fini(handle: *mut libzfs_handle_t);

    pub fn zfs_close(handle: *mut zfs_handle_t);

    // Error management
    pub fn libzfs_print_on_error(handle: *mut libzfs_handle_t, switch: c_bool);
    pub fn libzfs_errno(handle: *mut libzfs_handle_t) -> c_int;
    pub fn libzfs_error_action(handle: *mut libzfs_handle_t) -> *const c_char;
    pub fn libzfs_error_description(handle: *mut libzfs_handle_t) -> *const c_char;

    pub fn zfs_get_name(fs: *mut zfs_handle_t) -> *const c_char;
    pub fn zfs_path_to_zhandle(
        handle: *mut libzfs_handle_t,
        path: *mut c_char,
        path_type: zfs_type_t::Value,
    ) -> *mut zfs_handle_t;

    pub fn zfs_create(
        handle: *mut libzfs_handle_t,
        name: *const c_char,
        ztype: zfs_type_t::Value,
        props: *mut nvlist_t,
    ) -> c_int;
}
