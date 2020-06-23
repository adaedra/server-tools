use super::{native, Error, Zfs};
use log::debug;
use std::ffi::CString;
use std::fs::canonicalize;
use std::io;
use std::os::raw::c_char;
use std::path::Path;

pub struct Handle(pub(crate) *mut native::libzfs_handle_t);

impl Handle {
    pub fn new() -> io::Result<Handle> {
        debug!("Initializing ZFS library");
        let handle = unsafe { native::libzfs_init() };

        if handle.is_null() {
            Err(io::Error::last_os_error())
        } else {
            unsafe { native::libzfs_print_on_error(handle, 0) };
            Ok(Handle(handle))
        }
    }

    // In some case of errors (the most common ones...) libzfs will print an error message directly and not set it
    // in the handle.
    pub fn resolve<T: AsRef<Path>>(&self, path: T) -> Result<Zfs, Error> {
        // FIXME: This unwrap should not exist.
        let real_path = canonicalize(path).unwrap();
        let c_str = CString::new(real_path.to_str().unwrap()).unwrap();
        let handle = unsafe {
            native::zfs_path_to_zhandle(
                self.0,
                c_str.as_ptr() as *mut c_char,
                native::zfs_type_t::ZFS_TYPE_FILESYSTEM,
            )
        };

        if handle.is_null() {
            Err(Error::from_handle(self))
        } else {
            Ok(Zfs(handle))
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        debug!("Clearing ZFS library");
        unsafe { native::libzfs_fini(self.0) }
    }
}
