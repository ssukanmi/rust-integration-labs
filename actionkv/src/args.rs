use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub fname: String,
    pub action: String,
    pub key: String,
    pub maybe_value: Option<String>,
}
