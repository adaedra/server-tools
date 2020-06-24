use super::{native, Error, Library, NvList, Path};
use std::ffi::{CStr, CString};
use std::io;

pub struct Dataset(pub(crate) *mut native::zfs_handle_t);

impl Dataset {
    pub fn path(&self) -> Path {
        let spath = unsafe {
            let raw = native::zfs_get_name(self.0);
            CStr::from_ptr(raw).to_str().unwrap()
        };

        Path::new(spath)
    }

    pub fn create(handle: Library, path: Path) -> Result<Dataset, Error> {
        let props = NvList::new()?;
        let name = CString::new(path.to_string()).unwrap();

        let res = unsafe {
            native::zfs_create(
                handle.0,
                name.as_ptr(),
                native::zfs_type_t::ZFS_TYPE_FILESYSTEM,
                props.0,
            )
        };

        match res {
            0 => todo!(),
            err => Err(io::Error::from_raw_os_error(-err).into()),
        }
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        unsafe { native::zfs_close(self.0) };
    }
}
