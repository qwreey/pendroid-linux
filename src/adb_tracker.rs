use std::sync::Arc;

use adb_client::{ADBServer, DeviceShort, DeviceState, RustADBError};
use qwreey_utility_rs::RwMap;
use tokio::task::JoinHandle;

use crate::{
    DeviceMap,
    cli::{DeviceList, DeviceListUtil},
    connect_ws::connect_ws,
};

fn connected(userdata: &Arc<RwMap>, device: DeviceShort, port: i32) -> Result<(), RustADBError> {
    tracing::info!("Device connected: {}", device.identifier.as_str());

    // Forward server to local
    ADBServer::default()
        .get_device_by_name(device.identifier.as_str())?
        .forward(String::from("tcp:23227"), format!("tcp:{}", port))?;

    userdata.get_mut::<DeviceMap>("device_map").unwrap().insert(
        device.identifier.clone(),
        tokio::spawn(connect_ws(userdata.to_owned(), port, device)),
    );

    Ok(())
}

fn disconnected(userdata: &Arc<RwMap>, device: DeviceShort) {
    let mut map = userdata.get_mut::<DeviceMap>("device_map").unwrap();

    if let Some(task) = map.remove(device.identifier.as_str())
        && !task.is_finished()
    {
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            task.abort();
        });
    }
}

fn reset(userdata: &Arc<RwMap>) {
    userdata.get_mut::<DeviceMap>("device_map").unwrap().clear();
}

pub fn run_adb_tracker(userdata: Arc<RwMap>) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        loop {
            let userdata_clone = userdata.clone();
            reset(&userdata);
            let tracking = ADBServer::default().track_devices(move |device| {
                let state = device.state.clone() as i32;

                let list = userdata_clone.get_of::<DeviceList>().unwrap();
                if let Some(port) = list.get_port(&device.identifier) {
                    if state == DeviceState::Device as i32 {
                        connected(&userdata_clone, device, port)?;
                    } else if state == DeviceState::Offline as i32 {
                        tracing::info!("Device disconnected: {}", device.identifier.as_str());
                        disconnected(&userdata_clone, device);
                    }
                }
                Ok(())
            });
            if let Err(err) = tracking {
                tracing::error!("Error while tracking devices: {}", err);
            }
        }
    })
}
