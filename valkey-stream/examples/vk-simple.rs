//! A simple valkey/redis Set/Get (not stream)

use std::error::Error;

use redis::{Client, Commands};

#[tokio::main] // tokio async isn't required for this example
async fn main() -> Result<(), Box<dyn Error>> {
    // "redis://127.0.0.1/" (for default port 6379)
    let client = Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_connection()?;

    let _: () = conn.set("simple_key", 28)?;
    let res: i32 = conn.get("simple_key")?;

    println!("my_key result: {res}");

    Ok(())
}
