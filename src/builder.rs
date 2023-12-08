
use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct PrometheusMetrics {
    key: String,
    value: usize,
    #[builder(setter(into, strip_option), default)]
    help_str: Option<String>,
    #[builder(setter(into, strip_option), default)]
    metric_type: Option<String>,
    #[builder(default = "Vec::new()")]
    label: Vec<(String, String)>,
}

pub fn build_text(metrics: PrometheusMetrics) -> String {
    let mut result: String = String::new();
    let key = metrics.key;
    let value = metrics.value;

    if let Some(help_str) = metrics.help_str {
        result.push_str(&format!("# HELP {key} {help_str}\n"))
    }

    if let Some(metric_type) = metrics.metric_type {
        result.push_str(&format!("# TYPE {key} {metric_type}\n"))
    }

    let label_string = metrics
        .label
        .iter()
        .map(|(key, value)| format!("{key}=\"{value}\""))
        .collect::<Vec<String>>()
        .join(",");

    if !label_string.is_empty() {
        result.push_str(&format!("{key}{{{label_string},}} {value}\n"));
    } else {
        result.push_str(&format!("{key} {value}\n"));
    }
    return result;
}