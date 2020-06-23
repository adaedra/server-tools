use super::{native, Path};
use std::ffi::CStr;

pub struct Dataset(pub(crate) *mut native::zfs_handle_t);

impl Dataset {
    pub fn path(&self) -> Path {
        let spath = unsafe {
            let raw = native::zfs_get_name(self.0);
            CStr::from_ptr(raw).to_str().unwrap()
        };

        Path::new(spath)
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        unsafe { native::zfs_close(self.0) };
    }
}
