use regex::Regex;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

lazy_static! {
    static ref ADB_DEVICES_PATTERN: Regex = Regex::new(r"(\S+)\s+(device|offline|recovery|fastbootd|sideload|unauthorized|disconnected|bootloader)?\s*usb:(\S+)\s(?:product:(\S+)\s+)?(?:model:(\S+)\s+)?(?:device:(\S+)\s+)?transport_id:(\S+)").unwrap();
}

pub struct DeviceInfo {
    pub serial: String,
    pub state: String,
    pub model: String,
}

pub fn get_device_list() -> Vec<DeviceInfo> {
    let output = Command::new("adb")
        .arg("devices")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let reader = BufReader::new(output.stdout.unwrap());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let device_infos: Vec<DeviceInfo> = lines
        .iter()
        .filter_map(|line| ADB_DEVICES_PATTERN.captures(line))
        .map(|captures| DeviceInfo {
            serial: captures.get(1).map_or("", |m| m.as_str()).to_string(),
            state: captures.get(2).map_or("", |m| m.as_str()).to_string(),
            model: captures.get(5).map_or("", |m| m.as_str()).to_string(),
        })
        .collect();

    return device_infos;
}
