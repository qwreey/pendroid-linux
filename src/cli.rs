use qwreey_utility_rs::{ErrToString, HeadingError};

#[derive(clap::Parser, Clone)]
#[command(version, about)]
pub struct Command {
    #[arg(short, long, num_args = 1.., value_parser = parse_device)]
    pub devices: Vec<Device>,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(long)]
    pub daemon: bool,
    #[arg(long)]
    pub enable_autolaunch: bool,
    #[arg(long)]
    pub disable_autolaunch: bool,
    #[arg(long)]
    pub notify_connected: bool,
    #[arg(long)]
    pub notify_disconnected: bool,
}

const PARSE_ERROR: &str = "The device argument must be provided in the DeviceName:port format";

fn parse_device(arg: &str) -> Result<Device, String> {
    let mut split = arg.split(':');

    let name = split.next().ok_or(PARSE_ERROR).err_to_string()?.to_string();
    let port = split
        .next()
        .ok_or(PARSE_ERROR)?
        .parse::<i32>()
        .err_to_string()
        .heading_error("Failed to parse port number: ")?;

    if port <= 0 {
        return Err("Port number must be greater than 0".to_string());
    }

    Ok(Device {
        bind_port: port,
        name,
    })
}

#[derive(Clone)]
pub struct Device {
    pub bind_port: i32,
    pub name: String,
}

pub type DeviceList = Vec<Device>;
pub trait DeviceListUtil {
    fn get_port(&self, id: &str) -> Option<i32>;
}
impl DeviceListUtil for DeviceList {
    fn get_port(&self, id: &str) -> Option<i32> {
        for device in self {
            if device.name == id {
                return Some(device.bind_port);
            }
        }
        None
    }
}
