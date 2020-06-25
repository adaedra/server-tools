use super::{native, Library};
use log::trace;
use std::error::Error as RustError;
use std::ffi::CStr;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct ZfsError {
    errno: i32,
    action: Option<String>,
    description: String,
}

impl ZfsError {
    pub(crate) fn from_library(handle: &Library) -> ZfsError {
        trace!("Getting error");

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

        ZfsError {
            errno: unsafe { native::libzfs_errno(handle.0) },
            action,
            description,
        }
    }

    pub fn to_string(&self) -> String {
        match self.action {
            Some(ref action) => format!("{}: {}", action, self.description),
            None => self.description.to_owned(),
        }
    }
}

impl fmt::Display for ZfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl RustError for ZfsError {}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Zfs(ZfsError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(inner) => inner.fmt(f),
            Error::Zfs(inner) => inner.fmt(f),
        }
    }
}

impl RustError for Error {}

impl From<ZfsError> for Error {
    fn from(inner: ZfsError) -> Error {
        Error::Zfs(inner)
    }
}

impl From<io::Error> for Error {
    fn from(inner: io::Error) -> Error {
        Error::Io(inner)
    }
}
