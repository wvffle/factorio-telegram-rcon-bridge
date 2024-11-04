use std::sync::LazyLock;

use clap::{ArgGroup, Parser};
use dotenv::dotenv;

/// Factorio-Telegram bridge
#[derive(Parser)]
#[command(group = ArgGroup::new("log").required(true).args(&["factorio_log_file", "factorio_kube_namespace"]))]
pub struct Config {
    /// Path of state file that contains data shared between restarts
    #[arg(short = 's', long, env, default_value = "/tmp/cracktorio-bot")]
    pub state_file_path: String,

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
    #[arg(short = 'l', long, env, default_value = None, group = "log")]
    pub factorio_log_file: Option<String>,

    /// Factorio Kubernetes Namespace
    #[arg(short = 'n', long, env, default_value = None, group = "log")]
    pub factorio_kube_namespace: Option<String>,

    /// Factorio Kubernetes pod label filter
    #[arg(short = 'L', long, env, default_value = None)]
    pub factorio_kube_labels: Option<String>,

    /// Use Experimental factorio version
    #[arg(short = 'e', long, env = "FACTORIO_EXPERIMENTAL")]
    pub experimental: bool,

    /// If set, the bridge will check for new Factorio Friday Facts and send them to the chat.
    #[arg(long, env = "FACTORIO_FRIDAY_FACTS")]
    pub fff: bool,

    /// Retry connection if app crashed or server is down
    #[arg(short = 'r', long, env)]
    pub retry: bool,
}

impl Config {
    pub(crate) fn new() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);
