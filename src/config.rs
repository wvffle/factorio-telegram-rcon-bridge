use clap::Parser;
use dotenv::dotenv;

/// Factorio-Telegram bridge
#[derive(Parser)]
pub struct Config {
    /// Telegram bot token
    #[arg(short = 't', long, env)]
    pub telegram_token: String,

    /// Telegram chat id
    #[arg(short = 'c', long, env)]
    pub telegram_chat_id: i64,

    /// Factorio RCON host
    #[arg(short = 'H', long, env, default_value = "localhost:27015")]
    pub factorio_rcon_host: String,

    /// Factorio RCON password
    #[arg(short = 'P', long, env)]
    pub factorio_rcon_password: String,

    /// Factorio console log file
    #[arg(short = 'l', long, env)]
    pub factorio_log_file: String,

    /// Factorio Friday Facts Cache File. If set, the bridge will check for new FFFs and send them to the chat.
    #[arg(long, env, default_value = None)]
    pub fff: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self::parse()
    }
}
