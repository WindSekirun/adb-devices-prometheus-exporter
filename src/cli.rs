use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(about = "Prometheus exporter for monitoring adb devices")]
pub struct Cli {
    /// Port to listening metrics endpoint
    #[clap(default_value_t = 9001, short = 'p')]
    pub port: u32,

    /// Filtering devices that are in 'device' status but cannot be used
    #[clap(long, short, action, short = 'f')]
    pub filter_commend_available: bool
}