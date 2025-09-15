use std::{collections::HashMap, sync::Arc};

mod adb_tracker;
mod backend;
mod cli;
mod connect_ws;
mod logging;
mod parse;

use clap::Parser;
use cli::Command;

use daemonize::Daemonize;
use qwreey_utility_rs::{ErrToString, RwMap};
use tokio::task::JoinHandle;

pub type DeviceMap = HashMap<String, JoinHandle<()>>;

#[tokio::main]
async fn main() -> Result<(), String> {
    let userdata = Arc::new(RwMap::new());
    let command = Command::parse();
    userdata.insert_of(command.clone());

    if command.daemon {
        let daemonize = Daemonize::new();
        match daemonize.start() {
            Ok(_) => tracing::info!("Daemonized successfully"),
            Err(e) => tracing::error!("Error while daemonize: {}", e),
        }
    }

    logging::config_logger(command.verbose);

    userdata.insert_of(command.devices);
    userdata.insert("device_map", DeviceMap::new());

    adb_tracker::run_adb_tracker(userdata)
        .await
        .err_to_string()?;

    Ok(())
}
