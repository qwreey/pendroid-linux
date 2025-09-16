use std::{str::FromStr, sync::Arc};

use adb_client::DeviceShort;
use bytebuffer::{ByteReader, Endian};
use futures_util::StreamExt;
use http::Uri;
use notify_rust::{Notification};
use qwreey_utility_rs::RwMap;
use tokio::time::{Duration, sleep};
use tokio_websockets::ClientBuilder;

use crate::{backend::InputBackend, cli::Command, parse::Event};

fn process_buf(lazy_backend: &mut Option<InputBackend>, buf: &mut ByteReader) {
    let event = match Event::parse(buf) {
        Ok(event) => event,
        Err(err) => {
            tracing::error!("Failed to parse event: {}", err);
            return;
        }
    };

    // Init backend
    if let Event::Init(ref init) = event {
        match InputBackend::new(init) {
            Ok(backend) => {
                *lazy_backend = Some(backend);
            }
            Err(err) => {
                tracing::error!("Failed to initialize input backend: {}", err);
            }
        }

    // Execute command
    } else if let Some(backend) = lazy_backend {
        if let Err(err) = backend.execute(event) {
            tracing::error!("Input backend failed to execute command: {}", err);
        }

    // Backend not inited
    } else {
        tracing::warn!("Client send event before input backend initialization");
    }
}

pub async fn connect_ws(userdata: Arc<RwMap>, port: i32, device: DeviceShort) {
    let uri = Uri::from_str(format!("ws://127.0.0.1:{}", port).as_str()).unwrap();
    loop {
        if let Ok((mut client, _)) = ClientBuilder::from_uri(uri.clone()).connect().await {
            let mut lazy_backend: Option<InputBackend> = None;
            tracing::info!("Connected to ws://127.0.0.1:{}", port);

            // Show notification
            if userdata.get_of::<Command>().unwrap().notify_connected {
                let device_name = device.identifier.clone();
                tokio::spawn(async move {
                    let notification = Notification::new()
                        .summary("Pendroid Wired Connected")
                        .body(format!("Connected to {}", &device_name).as_str())
                        .appname("Pendroid Linux")
                        .timeout(5)
                        .show_async()
                        .await;
                    if let Err(err) = notification {
                        tracing::error!("Error while displaying notification: {}", err);
                    }
                });
            }

            // Get messages from server
            while let Some(item) = client.next().await {
                let msg = match item {
                    Ok(msg) => msg,
                    Err(err) => {
                        tracing::error!("Failed to read message: {}", err);
                        continue;
                    }
                };

                if !msg.is_binary() {
                    continue;
                }

                let mut buf = ByteReader::from_bytes(msg.as_payload());
                buf.set_endian(Endian::LittleEndian);
                process_buf(&mut lazy_backend, &mut buf);
            }

            tracing::info!("Disconnected from ws://127.0.0.1:{}", port);

            // Show notification
            if userdata.get_of::<Command>().unwrap().notify_disconnected {
                let device_name = device.identifier.clone();
                tokio::spawn(async move {
                    let notification = Notification::new()
                        .summary("Pendroid Wired Disconnected")
                        .body(format!("Disconnected from {}", &device_name).as_str())
                        .appname("Pendroid Linux")
                        .timeout(5)
                        .show_async()
                        .await;
                    if let Err(err) = notification {
                        tracing::error!("Error while displaying notification: {}", err);
                    }
                });
            }
        }

        sleep(Duration::from_secs(3)).await;
    }
}
