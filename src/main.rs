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
async fn run(command: Command) -> Result<(), String> {
    let userdata = Arc::new(RwMap::new());
    userdata.insert_of(command.clone());

    userdata.insert_of(command.devices);
    userdata.insert("device_map", DeviceMap::new());

    adb_tracker::run_adb_tracker(userdata)
        .await
        .err_to_string()?;

    Ok(())
}

fn main() -> Result<(), String> {
    let command = Command::parse();

    setup_logging::config(command.verbose);
    setup_autolaunch::config(command.enable_autolaunch, command.disable_autolaunch)?;
    setup_daemonize::config(command.daemon);

    run(command)
}
