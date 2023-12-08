use crate::{
    builder::{build_text, PrometheusMetricsBuilder},
    device::DeviceInfo,
};

fn kv(key: &str, value: &str) -> (String, String) {
    return (key.to_owned(), value.to_owned());
}

pub fn add_total_device_metrics(device_infos: &[DeviceInfo]) -> String {
    let mut result: String = String::new();

    let total_device = PrometheusMetricsBuilder::default()
        .key("connected_android_device".to_owned())
        .value(device_infos.len())
        .metric_type("gauge")
        .help_str("Information about Android devices connected to the host.")
        .build()
        .unwrap();

    result.push_str(&build_text(total_device));

    for device in device_infos {
        let mut label_vectors: Vec<(String, String)> = Vec::new();
        label_vectors.push(kv("serial", &device.serial));
        label_vectors.push(kv("model", &device.model));
        label_vectors.push(kv("state", &device.state));

        let each_metrics = PrometheusMetricsBuilder::default()
            .key("connected_android_device".to_owned())
            .value(1)
            .label(label_vectors)
            .build()
            .unwrap();

        result.push_str(&build_text(each_metrics));
    }

    return result;
}

pub fn add_device_metrics_state(device_infos: &[DeviceInfo], state: &str) -> String {
    let mut result: String = String::new();

    let value = device_infos
        .iter()
        .filter(|device| device.state == state)
        .count();

    let gauge = PrometheusMetricsBuilder::default()
        .key(format!("connected_android_device_state_{state}"))
        .value(value)
        .metric_type("gauge")
        .help_str(format!(
            "Number of Android devices connected in '{state}' status"
        ))
        .build()
        .unwrap();

    result.push_str(&build_text(gauge));
    return result;
}
