use clap::{clap_app, App};
use flexi_logger::Logger;
use log::{error, info};
use server_tools::root_dataset;
use std::process::exit;
use zfs::{Dataset, Zfs};

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
    let name = params.value_of("name").unwrap();

    let zfs = Zfs::new().unwrap();
    let container_root = {
        let root = root_dataset(&zfs).expect("Unable to open the root dataset");
        let mut path = root.path();

        path.push("containers");
        path.push(name);

        path
    };

    let dataset = match Dataset::create(&zfs, container_root) {
        Ok(dataset) => dataset,
        Err(error) => {
            error!("{}", error.to_string());
            exit(1);
        }
    };

    info!("Dataset {} created", dataset.path().to_string());

    if let Err(error) = dataset.create_child("root") {
        error!("{}", error.to_string());
        exit(1);
    }

    info!("Created root volume");
}
