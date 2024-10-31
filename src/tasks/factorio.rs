use crate::config::CONFIG;
use crate::Signal;

use color_eyre::eyre::Result;
use rcon::Connection;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::{error, info};

pub async fn run(tx: mpsc::Sender<Signal>) -> Result<()> {
    crate::log_reader::read_log(|line| async {
        if let Some((username, message)) = parse_log_line(line) {
            tx.send(Signal::MessageFromFactorio { username, message })
                .await
                .unwrap();
        }
    })
    .await?;

    Ok(())
}

pub async fn send(message: &str) {
    info!("[telegram] {}", message);

    let conn = <Connection<TcpStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(&CONFIG.factorio_rcon_host, &CONFIG.factorio_rcon_password)
        .await;

    match conn {
        Err(e) => error!("Could not connet to RCON: {}", e),
        Ok(mut conn) => {
            conn.cmd(message).await.unwrap();
        }
    }
}

async fn _cmd(message: &str) {
    info!("[  rcon  ] >> {}", message);

    let mut conn = <Connection<TcpStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(&CONFIG.factorio_rcon_host, &CONFIG.factorio_rcon_password)
        .await
        .expect("Failed to connect to RCON");

    let response = conn.cmd(message).await.unwrap();
    info!("[  rcon  ] << {}", response);
}

fn parse_log_line(line: String) -> Option<(String, String)> {
    // Server start
    if line.starts_with("=== Log opened") {
        return Some(("[server]".to_string(), "Starting...".to_string()));
    }

    // Server stop
    if line.starts_with("=== Log closed") {
        return Some(("[server]".to_string(), "Stopping...".to_string()));
    }

    // Join game
    if line.contains("[JOIN]") {
        let user = &line[27..].split(" ").next().unwrap();
        return Some(("[server]".to_string(), format!("{} joined the game", user)));
    }

    // Leave game
    if line.contains("[LEAVE]") {
        let user = &line[28..].split(" ").next().unwrap();
        return Some(("[server]".to_string(), format!("{} left the game", user)));
    }

    // Chat message
    if line.contains("[CHAT]") && line.contains("[gps=") == false {
        let user = &line[27..].split(" ").next().unwrap();
        let user = &user[0..user.len() - 1]; // Remove colon
        if user != "<server>" {
            let message = &line[27 + user.len() + 2..];
            return Some((user.to_string(), message.to_string()));
        }
    }

    None
}
