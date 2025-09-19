mod evdev;

#[derive(Clone)]
pub struct BackendConfig {
    pub evdev_trackpad_fuzz: i32,
    pub evdev_trackpad_res: i32,
    pub evdev_trackpad_flat: i32,
}

#[cfg(target_os = "linux")]
pub use evdev::InputBackend;
