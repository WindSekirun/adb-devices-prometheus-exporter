# adb-devices-prometheus-exporter
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Prometheus](https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=Prometheus&logoColor=white) [![Licence](https://img.shields.io/github/license/windsekirun/adb-devices-prometheus-exporter?style=for-the-badge)](./LICENSE) 

Prometheus exporter for expose 'adb devices' on host

## Metrics
* connected_android_device guage
* connected_android_device_state_(device|unauthorized|disconnected|sideload|recovery|offline|fastbood|bootloader) gauge

## Usage

for macOS and Homebrew users
```
brew tap windsekirun/tap
brew install adb-devices-prometheus-exporter

// manage by background service (launchctl on macos)
brew services start adb-devices-prometheus-exporter
```

access http://0.0.0.0:9001/metrics

## Build

* cargo run

## License
MIT License