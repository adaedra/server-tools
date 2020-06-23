use clap::{clap_app, App};
use flexi_logger::Logger;
use log::debug;
use server_tools::ROOT_DIR;
use zfs::Zfs;

fn app<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        (env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))

        (@arg name: * "Name of the container")
    )
}

fn main() {
    Logger::with_env().start().unwrap();
    let params = app().get_matches();
    debug!("Creating container {}", params.value_of("name").unwrap());

    let zfs = Zfs::new().unwrap();
    let root = zfs.resolve(ROOT_DIR).unwrap();
    debug!("{} is {}", ROOT_DIR, root.path());
}
