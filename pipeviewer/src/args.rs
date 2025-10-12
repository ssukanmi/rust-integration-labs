use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Read from a file instead of stdin
    pub infile: Option<String>,
    /// Write output to a file instead of stdout
    #[arg(short, long)]
    pub outfile: Option<String>,
    #[arg(short, long, env = "PV_SILENT")]
    pub silent: bool,
}
