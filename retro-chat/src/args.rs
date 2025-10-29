use clap::Parser;

/// Command-line arguments for retro chat app
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Username
    #[arg(short, long, default_value = "Guest")]
    pub name: String,

    /// Server address
    #[arg(short, long, default_value = "127.0.0.1:8082")]
    pub server: String,
}
