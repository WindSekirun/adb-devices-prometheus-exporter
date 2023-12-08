use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(about = "Prometheus exporter for monitoring adb devices")]
pub struct Cli {
    #[clap(default_value_t = 9001, short = 'p')]
    pub port: u32,
}