use std::error::Error;

use redis::{
    Client, Commands,
    streams::{StreamMaxlen, StreamReadOptions, StreamReadReply},
};

#[tokio::main] // tokio async isn't required for this example
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_connection()?;

    let stream_name = "xadd_stream";

    // -- Add entry to stream
    let id: String = conn.xadd(
        stream_name,
        "*",
        &[("firstname", "Tolu"), ("lastname", "Olayinka")],
    )?;
    println!("XADD id: {id}");

    // -- Read
    let res: StreamReadReply = conn.xread(&[stream_name], &["0"])?;
    println!("Entries:\n{res:#?}");

    // -- Read only one
    let optoins = StreamReadOptions::default().count(1);
    let res: StreamReadReply = conn.xread_options(&[stream_name], &[0], &optoins)?;
    println!("Single Entry:\n{res:#?}");

    // -- XTrim
    // Delete the entries of the stream
    let res: u64 = conn.xtrim(stream_name, StreamMaxlen::Equals(0))?;
    println!("xtrim count: {res}");

    // -- Del
    // Delete the key of the stream
    let res: u64 = conn.del(stream_name)?;
    println!("del count: {res}");

    Ok(())
}
