use super::native;
use std::ffi::CString;
use std::io::{Error, Result};
use std::os::raw::c_char;
use std::ptr::null_mut;

pub struct NvList(pub(crate) *mut native::nvlist_t);

pub trait NvType {
    const FUNCTION: native::nvlist_add_func<Self>;
}

impl NvType for i8 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_int8;
}

impl NvType for u8 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_uint8;
}

impl NvType for i16 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_int16;
}

impl NvType for u16 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_uint16;
}

impl NvType for i32 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_int32;
}

impl NvType for u32 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_uint32;
}

impl NvType for i64 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_int64;
}

impl NvType for u64 {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_uint64;
}

impl NvType for *const c_char {
    const FUNCTION: native::nvlist_add_func<Self> = native::nvlist_add_string;
}

impl NvList {
    pub fn new() -> Result<NvList> {
        let mut ptr = null_mut::<native::nvlist_t>();
        let res = unsafe {
            native::nvlist_alloc(
                &mut ptr as *mut *mut native::nvlist_t,
                native::NV_UNIQUE_NAME,
                0,
            )
        };

        match res {
            0 => Ok(NvList(ptr)),
            err => Err(Error::from_raw_os_error(-err)),
        }
    }

    fn add_native<T>(&self, name: &str, func: native::nvlist_add_func<T>, value: T) -> Result<()> {
        let cname = CString::new(name).unwrap();
        let res = unsafe { (func)(self.0, cname.as_ptr(), value.into()) };

        match res {
            0 => Ok(()),
            err => Err(Error::from_raw_os_error(-err)),
        }
    }

    #[allow(dead_code)]
    pub fn add<T: NvType, S: AsRef<str>>(&self, name: S, value: T) -> Result<()> {
        self.add_native(name.as_ref(), T::FUNCTION, value)
    }

    #[allow(dead_code)]
    pub fn add_bool<S: AsRef<str>>(&self, name: S, value: bool) -> Result<()> {
        self.add_native(
            name.as_ref(),
            native::nvlist_add_boolean_value,
            value.into(),
        )
    }

    #[allow(dead_code)]
    pub fn add_str<S: AsRef<str>>(&self, name: S, value: &str) -> Result<()> {
        let cstr = CString::new(value).unwrap();
        self.add_native(name.as_ref(), native::nvlist_add_string, cstr.as_ptr())
    }
}

impl Drop for NvList {
    fn drop(&mut self) {
        unsafe { native::nvlist_free(self.0) }
    }
}
