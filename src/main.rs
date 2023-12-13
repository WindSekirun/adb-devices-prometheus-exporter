pub mod builder;
pub mod cli;
pub mod device;
pub mod metrics;

#[macro_use]
extern crate lazy_static;
extern crate derive_builder;

use axum::{routing::get, Router};
use clap::Parser;
use device::{get_device_list, DeviceInfo};
use metrics::{add_total_device_metrics, add_device_metrics_state};

#[tokio::main]
async fn main() {
    // parse cli option
    let cli = cli::Cli::parse();
    let port: u32 = cli.port;

    // set router
    let app = Router::new()
        .route("/", get(hello))
        .route("/metrics", get(metrics));

    println!("Listening on 0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "access /metrics to access adb devices"
}

async fn metrics() -> String {
    let device_infos: Vec<DeviceInfo> = get_device_list();
    let mut result: String = String::new();

    result.push_str(&add_total_device_metrics(&device_infos));
    result.push_str(&add_device_metrics_state(&device_infos, "device"));
    result.push_str(&add_device_metrics_state(&device_infos, "unauthorized"));
    result.push_str(&add_device_metrics_state(&device_infos, "disconnected"));
    result.push_str(&add_device_metrics_state(&device_infos, "sideload"));
    result.push_str(&add_device_metrics_state(&device_infos, "recovery"));
    result.push_str(&add_device_metrics_state(&device_infos, "offline"));
    result.push_str(&add_device_metrics_state(&device_infos, "fastbootd"));
    result.push_str(&add_device_metrics_state(&device_infos, "bootloader"));
    result.push_str(&add_device_metrics_state(&device_infos, "FILTERED"));
    return result;
}