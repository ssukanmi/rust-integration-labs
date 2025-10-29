use anyhow::{Ok, Result};
use chrono::Local;
use retro_chat::message::{ChatMessage, MessageType};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8082").await?;

    // Display server startup message with formatting
    println!("╔════════════════════════════════════════╗");
    println!("║        RETRO CHAT SERVER ACTIVE        ║");
    println!("║        Port: 8082  Host: 127.0.0.1     ║");
    println!("║        Press Ctrl+C to shutdown        ║");
    println!("╚════════════════════════════════════════╝");

    let (tx, _) = broadcast::channel::<String>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("┌─[{}] New connection", Local::now().format("%H:%M:%S"));
        println!("└─ Address: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move { handle_connection(&mut socket, &tx, &mut rx).await });
    }
}

async fn handle_connection(
    socket: &mut TcpStream,
    tx: &broadcast::Sender<String>,
    rx: &mut broadcast::Receiver<String>,
) -> Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    let mut username = String::new();
    reader.read_line(&mut username).await?;
    let username = username.trim().to_string();

    // join notification
    let join_msg = ChatMessage {
        username: username.clone(),
        content: String::from("joined the chat"),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let join_json = serde_json::to_string(&join_msg)?;
    tx.send(join_json)?;

    // client messages
    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result? == 0 {
                    break;
                }
                let msg = ChatMessage {
                    username: username.clone(),
                    content: line.trim().to_string(),
                    timestamp: Local::now().format("%H:%M:%S").to_string(),
                    message_type: MessageType::UserMessage,
                };
                let json = serde_json::to_string(&msg)?;
                tx.send(json)?;
                line.clear();
            }
            result = rx.recv() => {
                let msg = result?;
                writer.write_all(msg.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }
    }

    // left notification
    let leave_msg = ChatMessage {
        username: username.clone(),
        content: "left the chat".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let leave_json = serde_json::to_string(&leave_msg)?;
    tx.send(leave_json)?;

    // Log
    println!(
        "└─[{}] {} disconnected",
        Local::now().format("%H:%M:%S"),
        username
    );

    Ok(())
}
