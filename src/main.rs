mod config;
mod factorio;
mod fff;
mod telegram;

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::new();

    let (telegram_tx, mut telegram_rx) = mpsc::channel::<String>(1);
    let (factorio_tx, mut factorio_rx) = mpsc::channel::<String>(1);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => { println!("Shutting down..."); },
        _ = tokio::spawn(factorio::tx(cfg.clone(), telegram_tx.clone())) => {},
        _ = tokio::spawn(telegram::tx(cfg.clone(), factorio_tx.clone())) => {},
        _ = factorio::rx(cfg.clone(), &mut factorio_rx) => {},
        _ = telegram::rx(cfg.clone(), &mut telegram_rx) => {},
        _ = fff::feed(cfg.clone(), vec![telegram_tx.clone(), factorio_tx.clone()]) => {},
    }

    Ok(())
}
