use std::os::raw::{c_char, c_int};

#[allow(non_camel_case_types)]
type c_bool = u32;

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
}
