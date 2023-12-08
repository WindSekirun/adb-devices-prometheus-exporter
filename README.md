# adb-devices-prometheus-exporter

Prometheus exporter for expose 'adb devices' on host

## Metrics
* connected_android_device guage
* connected_android_device_state_(device|unauthorized|disconnected|sideload|recovery|offline|fastbood|bootloader) gauge

## Build

* cargo run

access http://0.0.0.0:9001/metrics

## License
MIT License