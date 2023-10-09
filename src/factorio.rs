use crate::config::Config;
use async_log_watcher::LogWatcher;
use color_eyre::eyre::Result;
use rcon::Connection;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::info;

pub async fn send(cfg: &Config, message: &str) {
    info!("[telegram] {}", message);

    let mut conn = <Connection<TcpStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(&cfg.factorio_rcon_host, &cfg.factorio_rcon_password)
        .await
        .expect("Failed to connect to RCON");

    conn.cmd(message).await.unwrap();
}

async fn _cmd(cfg: &Config, message: &str) {
    info!("[  rcon  ] >> {}", message);

    let mut conn = <Connection<TcpStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(&cfg.factorio_rcon_host, &cfg.factorio_rcon_password)
        .await
        .expect("Failed to connect to RCON");

    let response = conn.cmd(message).await.unwrap();
    info!("[  rcon  ] << {}", response);
}

pub async fn rx(cfg: &Config, rx: &mut mpsc::Receiver<String>) {
    while let Some(message) = rx.recv().await {
        send(&cfg, &message).await;
    }
}

pub async fn tx(cfg: &Config, tx: mpsc::Sender<String>) -> Result<()> {
    let mut log_watcher = LogWatcher::new(cfg.factorio_log_file.clone());

    let handle = log_watcher.spawn(true);
    tokio::join!(async { handle.await.expect("Can't await handle") }, async {
        while let Some(data) = log_watcher.read_message().await {
            for line in std::str::from_utf8(&data).unwrap().split('\n') {
                if let Some(message) = read_log(line) {
                    tx.send(message).await.unwrap();
                }
            }
        }
    });

    Ok(())
}

fn read_log(line: &str) -> Option<String> {
    // Server start
    if line.starts_with("=== Log opened") {
        return Some("=== Server Started ===".to_string());
    }

    // Server stop
    if line.starts_with("=== Log closed") {
        return Some("=== Server Stopped ===".to_string());
    }

    // Join game
    if line.contains("[JOIN]") {
        let user = &line[27..].split(" ").next().unwrap();
        return Some(format!("{} joined the game", user));
    }

    // Leave game
    if line.contains("[LEAVE]") {
        let user = &line[28..].split(" ").next().unwrap();
        return Some(format!("{} left the game", user));
    }

    // Chat message
    if line.contains("[CHAT]") && line.contains("[gps=") == false {
        let user = &line[27..].split(" ").next().unwrap();
        let user = &user[0..user.len() - 1]; // NOTE: Remove colon
        if user != "<server>" {
            let message = &line[27 + user.len() + 2..];
            return Some(format!("{}: {}", user, message));
        }
    }

    None
}
