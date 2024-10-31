use feed_rs::parser;
use scraper::{Html, Selector};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::config::CONFIG;
use crate::state::STATE;
use crate::Signal;

pub async fn run(tx: mpsc::Sender<Signal>) {
    loop {
        check_version(&tx).await;
        check_fff(&tx).await;

        // Sleep 1 hour
        sleep(Duration::from_secs(60 * 60)).await;
    }
}

async fn check_fff(tx: &mpsc::Sender<Signal>) {
    if !CONFIG.fff {
        return;
    };

    let Ok(body) = reqwest::get("https://www.factorio.com/blog/rss").await else {
        return;
    };

    let Ok(xml) = body.bytes().await else {
        return;
    };

    let Ok(feed) = parser::parse_with_uri(xml.as_ref(), Some("https://www.factorio.com/blog/rss"))
    else {
        return;
    };

    let Some(post) = feed.entries.first() else {
        return;
    };

    let (Some(title), Some(link)) = (post.title.as_ref(), post.links.first()) else {
        return;
    };

    let last_title = STATE.lock().unwrap().latest_fff.clone();

    if last_title != title.content {
        {
            let mut state = STATE.lock().unwrap();
            let mut state = state.write();
            state.latest_fff = title.content.clone();
        }

        tx.send(Signal::FffUpdate {
            title: title.content.clone(),
            link: link.href.clone(),
        })
        .await
        .unwrap();
    }
}

async fn check_version(tx: &mpsc::Sender<Signal>) {
    let Ok(body) = reqwest::get("https://www.factorio.com/").await else {
        return;
    };

    let Ok(html) = body.text().await else {
        return;
    };

    let version_index = if CONFIG.experimental { 2 } else { 1 };

    let text = {
        let document = Html::parse_document(&html);
        let selector =
            Selector::parse(&format!(".box-releases dl:nth-of-type({version_index}) dd")).unwrap();

        let Some(ref element) = document.select(&selector).next() else {
            return;
        };

        element.text().collect::<Vec<_>>().join(" ")
    };

    let latest_version = text.trim();
    let (cached_version, current_version) = {
        let state = STATE.lock().unwrap();
        (
            state.latest_factorio_version.clone(),
            state.current_factorio_version.clone(),
        )
    };

    if cached_version != latest_version && current_version != latest_version {
        {
            let mut state = STATE.lock().unwrap();
            let mut state = state.write();
            state.latest_factorio_version = latest_version.to_string();
        }

        tx.send(Signal::VersionUpdate(latest_version.to_string()))
            .await
            .unwrap();
    }
}
