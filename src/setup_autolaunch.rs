use auto_launch::AutoLaunch;
use qwreey_utility_rs::{ErrToString, HeadingError};
use std::{env::args, fs::canonicalize, process::exit};

pub fn config(enable_autolaunch: bool, disable_autolaunch: bool) -> Result<(), String> {
    if !enable_autolaunch && !disable_autolaunch {
        return Ok(());
    }

    let mut args_iter = args();
    let path = args_iter.next().unwrap();
    let path = canonicalize(path)
        .err_to_string()
        .heading_error("To enable autolaunch, execute pendroid-linux with an absolute path: ")?;
    let path = path.to_str().unwrap();

    let args_vec: Vec<String> = args_iter
        .filter(|this| this != "--enable-autolaunch" && this != "--disable-autolaunch")
        .map(|str| format!("\"{}\"", str.replace("\"", "\"\"")))
        .collect();

    let auto = AutoLaunch::new("pendroid-linux", path, args_vec.as_slice());
    if enable_autolaunch {
        auto.enable().err_to_string()?;
        tracing::info!("Auto-Launch enabled");
    } else if disable_autolaunch {
        auto.disable().err_to_string()?;
        tracing::info!("Auto-Launch disabled");
    }

    exit(0);
}
