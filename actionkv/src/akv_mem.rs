use std::path::Path;

use anyhow::Result;
use clap::Parser;
use libactionkv::{ActionKV, args::Args};

#[cfg(target_os = "windows")]
const USAGE: &str = "
    Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() -> Result<()> {
    let Args {
        fname,
        action,
        key,
        maybe_value,
    } = Args::parse();

    let path = Path::new(&fname);

    let mut store = ActionKV::open(path)?;
    store.load()?;

    match action.as_ref() {
        "get" => match store.get(key.as_ref())? {
            Some(value) => println!("{:?}", value),
            None => eprintln!("{:?} not found", key),
        },
        "delete" => store.delete(key.as_ref())?,
        "insert" => {
            if let Some(value) = maybe_value {
                store.insert(key.as_ref(), value.as_ref())?;
            } else {
                eprintln!("{}", &USAGE);
            }
        }
        "update" => {
            if let Some(value) = maybe_value {
                store.update(key.as_ref(), value.as_ref())?;
            } else {
                eprintln!("{}", &USAGE);
            }
        }
        _ => eprintln!("{}", &USAGE),
    }

    Ok(())
}
