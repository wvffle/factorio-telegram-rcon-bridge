#![feature(lazy_cell)]

use std::sync::LazyLock;

mod config;
mod factorio;
mod fff;
mod telegram;

use color_eyre::eyre::Result;
use config::Config;
use tokio::sync::mpsc;
use tracing::info;

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let (telegram_tx, mut telegram_rx) = mpsc::channel::<String>(1);
    let (factorio_tx, mut factorio_rx) = mpsc::channel::<String>(1);

    tokio::task::spawn(fff::feed(
        &*CONFIG,
        vec![telegram_tx.clone(), factorio_tx.clone()],
    ));

    tokio::select! {
        _ = tokio::signal::ctrl_c() => { info!("Shutting down..."); },
        _ = tokio::spawn(factorio::tx(&*CONFIG, telegram_tx.clone())) => {
            info!("Factorio tx task finished");
        },
        _ = tokio::spawn(telegram::tx(&*CONFIG, factorio_tx.clone())) => {
            info!("Telegram tx task finished");
        },
        _ = factorio::rx(&*CONFIG, &mut factorio_rx) => {
            info!("Factorio rx task finished");
        },
        _ = telegram::rx(&*CONFIG, &mut telegram_rx) => {
            info!("Telegram rx task finished");
        },
    }

    Ok(())
}
