use std::{collections::HashMap, sync::Arc};

mod adb_tracker;
mod backend;
mod cli;
mod connect_ws;
mod parse;
mod setup_autolaunch;
mod setup_daemonize;
mod setup_logging;

use clap::Parser;
use cli::Command;

use qwreey_utility_rs::{ErrToString, RwMap};
use tokio::task::JoinHandle;

pub type DeviceMap = HashMap<String, JoinHandle<()>>;

#[tokio::main]
async fn main() -> Result<(), String> {
    let userdata = Arc::new(RwMap::new());
    let command = Command::parse();
    userdata.insert_of(command.clone());

    setup_autolaunch::config(command.enable_autolaunch, command.disable_autolaunch)?;
    setup_daemonize::config(command.daemon);
    setup_logging::config(command.verbose);

    userdata.insert_of(command.devices);
    userdata.insert("device_map", DeviceMap::new());

    adb_tracker::run_adb_tracker(userdata)
        .await
        .err_to_string()?;

    Ok(())
}
