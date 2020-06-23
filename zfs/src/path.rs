use std::fmt::{self, Display, Formatter};
use std::path;

#[derive(Debug)]
pub struct Path(String, path::PathBuf);

impl Path {
    pub fn new<T: AsRef<str>>(path: T) -> Path {
        let rpath = path.as_ref();
        let pos = rpath.find('/');

        match pos {
            Some(pos) => {
                let (pool_name, zfs_path) = rpath.split_at(pos);
                Path(pool_name.to_owned(), path::PathBuf::from(zfs_path))
            }
            None => Path(rpath.to_owned(), path::PathBuf::from("/")),
        }
    }

    pub fn pool_name(&self) -> &str {
        &self.0
    }

    pub fn path(&self) -> &path::Path {
        &self.1
    }

    pub fn to_string(&self) -> String {
        if self.1.parent().is_none() {
            self.0.to_owned()
        } else {
            format!("{}{}", self.0, self.1.display())
        }
    }
}

impl Display for Path {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}
