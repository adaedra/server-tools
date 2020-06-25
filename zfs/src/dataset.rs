use super::{native, Error, Library, NvList, Path, ZfsError};
use log::trace;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::path;

pub struct Dataset(pub(crate) *mut native::zfs_handle_t);

impl Dataset {
    // Since we get a copy of the pointer to make our own Library, we don't want to drop it.
    pub fn library(&self) -> ManuallyDrop<Library> {
        trace!("zfs_get_handle");
        let handle = unsafe { native::zfs_get_handle(self.0) };
        ManuallyDrop::new(Library(handle))
    }

    pub fn path(&self) -> Path {
        let spath = unsafe {
            trace!("zfs_get_name");
            let raw = native::zfs_get_name(self.0);
            CStr::from_ptr(raw).to_str().unwrap()
        };

        Path::new(spath)
    }

    pub fn create(handle: &Library, path: Path) -> Result<Dataset, Error> {
        let props = NvList::new()?;
        let name = CString::new(path.to_string()).unwrap();

        trace!("zfs_create {:?}", name);
        let res = unsafe {
            native::zfs_create(
                handle.0,
                name.as_ptr(),
                native::zfs_type_t::ZFS_TYPE_FILESYSTEM,
                props.0,
            )
        };

        match res {
            0 => Self::open(handle, path),
            _ => Err(ZfsError::from_library(handle).into()),
        }
    }

    pub fn create_child<T: AsRef<str>>(&self, name: T) -> Result<Dataset, Error> {
        let library = self.library();
        let mut path = self.path().clone();

        path.push(name.as_ref());

        Self::create(&*library, path)
    }

    // In some case of errors (the most common ones...) libzfs will print an error message directly and not set it
    // in the handle.
    pub fn resolve<T: AsRef<path::Path>>(library: &Library, path: T) -> Result<Dataset, Error> {
        use std::fs::canonicalize;

        let real_path = canonicalize(path)?;
        let c_str = CString::new(real_path.to_str().unwrap()).unwrap();

        trace!("zfs_path_to_zhandle {:?}", c_str);
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

    pub fn open(library: &Library, path: Path) -> Result<Dataset, Error> {
        let cpath = CString::new(path.to_string()).unwrap();
        let handle = unsafe {
            trace!("zfs_open {:?}", cpath);
            native::zfs_open(
                library.0,
                cpath.as_ptr(),
                native::zfs_type_t::ZFS_TYPE_FILESYSTEM as i32,
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
        trace!("zfs_close");
        unsafe { native::zfs_close(self.0) };
    }
}
