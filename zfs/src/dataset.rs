use super::{native, Error, Library, NvList, Path, ZfsError};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{io, path};

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

    // In some case of errors (the most common ones...) libzfs will print an error message directly and not set it
    // in the handle.
    pub fn resolve<T: AsRef<path::Path>>(library: &Library, path: T) -> Result<Dataset, Error> {
        use std::fs::canonicalize;

        let real_path = canonicalize(path)?;
        let c_str = CString::new(real_path.to_str().unwrap()).unwrap();
        let handle = unsafe {
            native::zfs_path_to_zhandle(
                library.0,
                c_str.as_ptr() as *mut c_char,
                native::zfs_type_t::ZFS_TYPE_FILESYSTEM,
            )
        };

        if handle.is_null() {
            Err(ZfsError::from_library(library).into())
        } else {
            Ok(Dataset(handle))
        }
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        unsafe { native::zfs_close(self.0) };
    }
}
