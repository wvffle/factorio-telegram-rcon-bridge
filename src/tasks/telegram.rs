use color_eyre::eyre::Result;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

use crate::{config::CONFIG, Signal};

pub async fn send(message: &str) {
    info!("[factorio] {}", message);

    let bot = teloxide::Bot::new(CONFIG.telegram_token.clone());
    let err = bot
        .send_message(ChatId(CONFIG.telegram_chat_id), message)
        .await;

    if let Err(e) = err {
        error!("Error sending message: {}", e);
    }
}

pub async fn run(tx: mpsc::Sender<Signal>) -> Result<()> {
    let bot = teloxide::Bot::new(CONFIG.telegram_token.clone());
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

            if message.is_none() && caption.is_none() && !has_image {
                return Ok(());
            }

            // Get username &str from user.unwrap().username or set it to "[TELEGRAM]" if it's None
            let username: &str = user.unwrap().username.as_deref().unwrap_or("[TELEGRAM]");

            let tx = tx.lock().await;
            if has_image {
                tx.send(Signal::MessageFromTelegram {
                    username: username.to_string(),
                    message: "[img]".to_string(),
                })
                .await
                .unwrap();
            }

            if let Some(message) = message {
                tx.send(Signal::MessageFromTelegram {
                    username: username.to_string(),
                    message: message.to_string(),
                })
                .await
                .unwrap();
            }

            if let Some(message) = caption {
                tx.send(Signal::MessageFromTelegram {
                    username: username.to_string(),
                    message: message.to_string(),
                })
                .await
                .unwrap();
            }

            Ok(())
        }
    })
    .await;

    Ok(())
}
