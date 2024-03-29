use clap::Parser;
use regex::Regex;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

use crate::cli;

lazy_static! {
    // adb version 33.0.3-8952118
    static ref ADB_DEVICES_PATTERN: Regex = Regex::new(r"(\S+)\s+(device|offline|recovery|fastbootd|sideload|unauthorized|disconnected|bootloader)?\s*usb:(\S+)\s(?:product:(\S+)\s+)?(?:model:(\S+)\s+)?(?:device:(\S+)\s+)?transport_id:(\S+)").unwrap();
    // adb_version 34.0.5-10900879
    static ref ADB_DEVICES_PATTERN_34: Regex = Regex::new(r"(\S+)\s+(device|offline|recovery|fastbootd|sideload|unauthorized|disconnected|bootloader)?\s*(\S+)\s(?:product:(\S+)\s+)?(?:model:(\S+)\s+)?(?:device:(\S+)\s+)?transport_id:(\S+)").unwrap();
}

#[derive(Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub state: String,
    pub model: String,
}

pub fn get_device_list() -> Vec<DeviceInfo> {
    let mut child = Command::new("adb")
        .arg("devices")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let one_sec = Duration::from_secs(5);

    let output = match child
        .wait_timeout(one_sec)
        .expect("Failed to wait for adb command")
    {
        Some(status) if status.success() => {
            child.wait_with_output().expect("Failed to read stdout")
        }
        Some(_) | None => {
            child
                .kill()
                .expect("Command wasn't running or couldn't be killed");
            return Vec::new(); // Exit code was not 0 or timeout happened, return empty Vec
        }
    };

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert stdout to String");

    let lines: Vec<String> = output_str.lines().map(|l| l.to_string()).collect();
    let device_infos: Vec<DeviceInfo> = lines
        .iter()
        .filter_map(|line| {
            ADB_DEVICES_PATTERN
                .captures(line)
                .or_else(|| ADB_DEVICES_PATTERN_34.captures(line))
                .map(|captures| DeviceInfo {
                    serial: captures.get(1).map_or("", |m| m.as_str()).to_string(),
                    state: captures.get(2).map_or("", |m| m.as_str()).to_string(),
                    model: captures.get(5).map_or("", |m| m.as_str()).to_string(),
                })
        })
        .map(|info| check_command_available(&info))
        .collect();

    return device_infos;
}

fn check_command_available(device_info: &DeviceInfo) -> DeviceInfo {
    let cli = cli::Cli::parse();
    let filter_command_available = cli.filter_commend_available;

    if !filter_command_available {
        return device_info.clone();
    }

    // Doesn't filter when device has not 'device' state (unauthorized.. etc)
    if device_info.state != "device" {
        return device_info.clone();
    }

    let mut child: std::process::Child = Command::new("adb")
        .arg("-s")
        .arg(&device_info.serial)
        .arg("shell")
        .arg("getprop")
        .arg("ro.product.model")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let one_sec = Duration::from_secs(1);

    let status_code = match child.wait_timeout(one_sec).unwrap() {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            child.wait().unwrap().code()
        }
    };

    if status_code.is_some() && status_code.unwrap() != 0 {
        let mut new_info = device_info.clone();
        new_info.state = "FILTERED".to_owned();
        return new_info;
    }

    return device_info.clone();
}
