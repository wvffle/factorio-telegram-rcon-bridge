use crate::config::Config;
use color_eyre::eyre::Result;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

async fn send(cfg: &Config, message: &str) {
    info!("[factorio] {}", message);

    let bot = teloxide::Bot::new(cfg.telegram_token.clone());
    let err = bot
        .send_message(ChatId(cfg.telegram_chat_id.clone()), message)
        .await;

    if let Err(e) = err {
        error!("Error sending message: {}", e);
    }
}

pub async fn rx(cfg: &Config, rx: &mut mpsc::Receiver<String>) {
    while let Some(message) = rx.recv().await {
        send(&cfg, &message).await;
    }
}

pub async fn tx(cfg: &Config, tx: mpsc::Sender<String>) -> Result<()> {
    let bot = teloxide::Bot::new(cfg.telegram_token.clone());
    let tx = Arc::new(Mutex::new(tx.clone()));

    teloxide::repl(bot, move |_bot: Bot, msg: Message| {
        let tx = tx.clone();
        async move {
            let user = msg.from();
            let message = msg.text();
            let caption = msg.caption();

            if user.is_none() {
                return Ok(());
            }

            let mut has_image = false;
            if msg.photo().is_some() || msg.video().is_some() {
                has_image = true;
            }

            if message.is_none() && caption.is_none() && has_image == false {
                return Ok(());
            }

            // Get username &str from user.unwrap().username or set it to "[TELEGRAM]" if it's None
            let username: &str = user
                .unwrap()
                .username
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("[TELEGRAM]");

            let tx = tx.lock().await;
            if has_image {
                let msg = format!("{}: {}", username, "[img]");
                tx.send(msg).await.unwrap();
            }

            if message.is_some() {
                let msg = format!("{}: {}", username, message.unwrap());
                tx.send(msg).await.unwrap();
            }

            if caption.is_some() {
                let msg = format!("{}: {}", username, caption.unwrap());
                tx.send(msg).await.unwrap();
            }

            Ok(())
        }
    })
    .await;

    Ok(())
}
