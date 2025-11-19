use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub url: String,

    #[arg(long)]
    pub tap_device: String,

    #[arg(long, default_value = "1.1.1.1")]
    pub dns_server: String,
}
