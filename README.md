# adb-devices-prometheus-exporter
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Prometheus](https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=Prometheus&logoColor=white) ![Homebrew](https://img.shields.io/badge/dynamic/json.svg?url=https://raw.githubusercontent.com/windsekirun/homebrew-tap/master/Info/adb-devices-prometheus-exporter.json&query=$.versions.stable&label=homebrew&style=for-the-badge&logo=homebrew&logoColor=white) [![Licence](https://img.shields.io/github/license/windsekirun/adb-devices-prometheus-exporter?style=for-the-badge)](./LICENSE)

Prometheus exporter for expose 'adb devices' on host

## Metrics
```
# HELP connected_android_device Information about Android devices connected to the host.
# TYPE connected_android_device gauge
connected_android_device 0

# HELP connected_android_device_state_device Number of Android devices connected in 'device' status
# TYPE connected_android_device_state_device gauge
connected_android_device_state_device 0

# HELP connected_android_device_state_unauthorized Number of Android devices connected in 'unauthorized' status
# TYPE connected_android_device_state_unauthorized gauge
connected_android_device_state_unauthorized 0

# HELP connected_android_device_state_disconnected Number of Android devices connected in 'disconnected' status
# TYPE connected_android_device_state_disconnected gauge
connected_android_device_state_disconnected 0

# HELP connected_android_device_state_sideload Number of Android devices connected in 'sideload' status
# TYPE connected_android_device_state_sideload gauge
connected_android_device_state_sideload 0

# HELP connected_android_device_state_recovery Number of Android devices connected in 'recovery' status
# TYPE connected_android_device_state_recovery gauge
connected_android_device_state_recovery 0

# HELP connected_android_device_state_offline Number of Android devices connected in 'offline' status
# TYPE connected_android_device_state_offline gauge
connected_android_device_state_offline 0

# HELP connected_android_device_state_fastbootd Number of Android devices connected in 'fastbootd' status
# TYPE connected_android_device_state_fastbootd gauge
connected_android_device_state_fastbootd 0

# HELP connected_android_device_state_bootloader Number of Android devices connected in 'bootloader' status
# TYPE connected_android_device_state_bootloader gauge
connected_android_device_state_bootloader 0

# HELP connected_android_device_state_filtered Number of Android devices connected in 'FILTERED' status
# TYPE connected_android_device_state_filtered gauge
connected_android_device_state_filtered 0
```

* `connected_android_device_state_filtered` is available with -f options in v0.2.1 or newer

## Usage

```
Prometheus exporter for monitoring adb devices

Usage: adb-devices-prometheus-exporter [OPTIONS]

Options:
  -p <PORT>                       Port to listening metrics endpoint [default: 9001]
  -f, --filter-commend-available  Filtering devices that are in 'device' status but cannot be used
  -h, --help                      Print help
  -V, --version                   Print version
```

### Install

for macOS and Homebrew users
```
brew tap windsekirun/tap
brew install adb-devices-prometheus-exporter

// manage by background service (launchctl on macos)
brew services start adb-devices-prometheus-exporter

// or, use launchctl instead
sh install-plist-macos.sh
```

access http://0.0.0.0:9001/metrics

## Build

* cargo run

## License
MIT License