use anyhow::Result;
use clap::Parser;
use libactionkv::{ActionKV, args::Args};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
 ";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) -> Result<()> {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index)?;
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes)?;

    Ok(())
}

fn main() -> Result<()> {
    const INDEX_KEY: &ByteStr = b"+index";

    let Args {
        fname,
        action,
        key,
        maybe_value,
    } = Args::parse();

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path)?;
    let key = key.as_ref();

    a.load()?;

    match action.as_ref() {
        "get" => {
            let index_as_bytes = a.get(INDEX_KEY)?.unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);

            let index: HashMap<ByteString, u64> = index_decoded?;

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = a.get_at(i)?;
                    println!("{:?}", kv.value)
                }
            }
        }

        "delete" => a.delete(key)?,

        "insert" => {
            if let Some(value) = maybe_value {
                a.insert(key, value.as_ref())?;
                store_index_on_disk(&mut a, INDEX_KEY)?;
            } else {
                eprintln!("{}", &USAGE);
            }
        }

        "update" => {
            if let Some(value) = maybe_value {
                a.update(key, value.as_ref())?;
                store_index_on_disk(&mut a, INDEX_KEY)?;
            } else {
                eprintln!("{}", &USAGE);
            }
        }
        _ => eprintln!("{}", &USAGE),
    }

    Ok(())
}
