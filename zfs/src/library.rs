use super::native;
use log::debug;
use std::io;

pub struct Library(pub(crate) *mut native::libzfs_handle_t);

impl Library {
    pub fn new() -> io::Result<Library> {
        debug!("Initializing ZFS library");
        let handle = unsafe { native::libzfs_init() };

        if handle.is_null() {
            Err(io::Error::last_os_error())
        } else {
            unsafe { native::libzfs_print_on_error(handle, 0) };
            Ok(Library(handle))
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        debug!("Clearing ZFS library");
        unsafe { native::libzfs_fini(self.0) }
    }
}
