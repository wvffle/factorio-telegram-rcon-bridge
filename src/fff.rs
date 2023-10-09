use feed_rs::parser;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::error;

use crate::config::Config;

pub async fn feed(cfg: &Config, txs: Vec<mpsc::Sender<String>>) {
    if cfg.fff.is_none() {
        return;
    }

    let fff_cache_file = &cfg.fff.clone().unwrap();

    loop {
        let Ok(body) = reqwest::get("https://www.factorio.com/blog/rss").await else {
            continue;
        };

        let Ok(xml) = body.bytes().await else {
            continue;
        };

        let Ok(feed) =
            parser::parse_with_uri(xml.as_ref(), Some("https://www.factorio.com/blog/rss"))
        else {
            continue;
        };

        let Some(post) = feed.entries.first() else {
            continue;
        };

        let (Some(title), Some(link)) = (post.title.as_ref(), post.links.first()) else {
            continue;
        };

        let last_title = std::fs::read_to_string(fff_cache_file).unwrap_or("".to_string());
        if last_title != title.content {
            match std::fs::write(fff_cache_file, title.content.clone()) {
                Ok(_) => {}
                Err(e) => error!("Error writing to file: {}", e),
            }

            for tx in txs.iter() {
                tx.send(format!("Factorio {}\n{}", title.content, link.href))
                    .await
                    .unwrap();
            }
        }

        // Sleep 1 hour
        sleep(Duration::from_secs(60 * 60)).await;
    }
}
