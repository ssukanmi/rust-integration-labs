use std::{error::Error, time::Duration};

use redis::{
    AsyncCommands, Client,
    streams::{StreamReadOptions, StreamReadReply},
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::open("redis://127.0.0.1:6379")?;

    let stream_name = "async_stream";

    // -- Writer Task
    let mut conn_writer = client.get_multiplexed_async_connection().await?;
    let writer_handle = tokio::spawn(async move {
        println!("WRITER - started");
        for i in 0..5 {
            let id: String = conn_writer
                .xadd(stream_name, "*", &[("val", &i.to_string())])
                .await
                .expect("xadd failed");
            println!("WRITER - sent 'val: {i}' with id: {id}");
            sleep(Duration::from_millis(200)).await;
        }
    });

    // -- Reader Task
    let mut conn_reader = client.get_multiplexed_async_connection().await?;
    let reader_handle = tokio::spawn(async move {
        println!("READER - started");
        let mut last_id = "0-0".to_string();

        loop {
            let options = StreamReadOptions::default().count(1).block(2000);
            let res: Option<StreamReadReply> = conn_reader
                .xread_options(&[stream_name], &[&last_id], &options)
                .await
                .expect("Failed to xread");

            if let Some(reply) = res {
                for stream_key in reply.keys {
                    for stream_id in stream_key.ids {
                        println!(
                            "READER - read: id: {} - fields: {:?}",
                            stream_id.id, stream_id.map
                        );
                        println!("READER - SLEEP 800MS");
                        sleep(Duration::from_millis(800)).await;

                        last_id = stream_id.id;
                    }
                }
            } else {
                println!("READER -  timeout, assuming writer is done.");
                break;
            }
        }
        println!("READER - finished");
    });

    // -- Wait for the tasks
    writer_handle.await?;
    reader_handle.await?;

    // -- Clean up the stream
    let mut conn = client.get_multiplexed_async_connection().await?;
    let count: i32 = conn.del(stream_name).await?;
    println!("Stream '{stream_name}' deleted ({count} key).");

    Ok(())
}
