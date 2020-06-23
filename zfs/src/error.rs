use super::{native, Zfs};
use std::ffi::CStr;

#[derive(Debug)]
pub struct Error {
    errno: i32,
    action: Option<String>,
    description: String,
}

impl Error {
    pub(crate) fn from_library(handle: &Zfs) -> Error {
        let action = unsafe {
            let raw = native::libzfs_error_action(handle.0);
            let cstr = CStr::from_ptr(raw).to_str().unwrap();

            if cstr.len() > 0 {
                Some(cstr.to_owned())
            } else {
                None
            }
        };
        let description = unsafe {
            let raw = native::libzfs_error_description(handle.0);
            CStr::from_ptr(raw).to_str().unwrap().to_owned()
        };

        Error {
            errno: unsafe { native::libzfs_errno(handle.0) },
            action,
            description,
        }
    }
}
