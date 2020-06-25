use log::{debug, warn};
use zfs::{Dataset, Error, Zfs};

pub const ROOT_DIR: &'static str = "/var/lib/server-tools";

pub fn root_dataset(handle: &Zfs) -> Result<Dataset, Error> {
    let res = Dataset::resolve(handle, ROOT_DIR);

    match res {
        Ok(ref dataset) => debug!("{} is {}", ROOT_DIR, dataset.path().to_string()),
        Err(ref error) => warn!("Could not resolve dataset at {}", error),
    }

    res
}
