mod config;
mod log_reader;
mod state;
mod tasks;

use color_eyre::eyre::Result;
use tokio::sync::mpsc;
use tracing::{error, info};

pub enum Signal {
    MessageFromFactorio { username: String, message: String },
    MessageFromTelegram { username: String, message: String },
    FffUpdate { title: String, link: String },
    VersionUpdate(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel::<Signal>(1);

    let scrapper = tasks::scrapper::run(tx.clone());
    let factorio = tasks::factorio::run(tx.clone());
    let telegram = tasks::telegram::run(tx.clone());

    tokio::select! {
        _ = tokio::signal::ctrl_c() => { info!("Shutting down..."); },
        result = tokio::spawn(scrapper) => {
            match result {
                Err(e) => {
                    error!("Scrapper task failed: {}", e);
                }
                Ok(()) => {
                    info!("Scrapper task finished");
                }
            }
        },

        result = tokio::spawn(factorio) => {
            match result {
                Err(e) => {
                    error!("Factorio log reading task failed: {}", e);
                }
                Ok(Err(e)) => {
                    error!("Factorio log reading task failed: {}", e);
                }
                Ok(Ok(())) => {
                    info!("Factorio log reading task finished");
                }
            }
        },
        result = tokio::spawn(telegram) => {
            match result {
                Err(e) => {
                    error!("Telegram message receiving task failed: {}", e);
                }
                Ok(Err(e)) => {
                    error!("Telegram message receiving task failed: {}", e);
                }
                Ok(Ok(())) => {
                    info!("Telegram message receiving task finished");
                }
            }
        },
        result = tokio::spawn(bridge(rx)) => {
            match result {
                Err(e) => {
                    error!("Bridge task failed: {}", e);
                }
                Ok(()) => {
                    info!("Bridge task finished");
                }
            }
        },
    }

    Ok(())
}

async fn bridge(mut rx: mpsc::Receiver<Signal>) {
    loop {
        match rx.recv().await.unwrap() {
            Signal::MessageFromFactorio { username, message } => {
                tasks::telegram::send(&format!("{}: {}", username, message)).await;
            }

            Signal::MessageFromTelegram { username, message } => {
                tasks::factorio::send(&format!("{}: {}", username, message)).await;
            }

            Signal::FffUpdate { title, link } => {
                tasks::telegram::send(&format!("{}\n\n{}", title, link)).await;
                tasks::factorio::send(&format!("FFF: {}", title)).await;
            }

            Signal::VersionUpdate(version) => {
                tasks::telegram::send(&format!(
                    "Factorio {} is released! @wvffle update the server.",
                    version
                ))
                .await;
            }
        }
    }
}
