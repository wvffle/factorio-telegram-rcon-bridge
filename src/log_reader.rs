use std::future::Future;

use async_log_watcher::LogWatcher;
use color_eyre::eyre::Result;
use futures::{AsyncBufReadExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{ListParams, LogParams},
    Api, Client, ResourceExt,
};

use crate::config::CONFIG;

pub async fn read_log<F, Fut>(line_reader: F) -> Result<()>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    if let Some(ref log_file) = CONFIG.factorio_log_file {
        file_log_reader(log_file.clone(), line_reader).await;
        return Ok(());
    }

    if let Some(ref namespace) = CONFIG.factorio_kube_namespace {
        kube_log_reader(
            namespace.clone(),
            CONFIG.factorio_kube_labels.clone(),
            line_reader,
        )
        .await?;
        return Ok(());
    }

    panic!("No log source provided");
}

async fn file_log_reader<F, Fut>(log_file: String, line_reader: F)
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    let mut log_watcher = LogWatcher::new(log_file);
    let handle = log_watcher.spawn(true);

    tokio::join!(async { handle.await.expect("Can't await handle") }, async {
        while let Some(data) = log_watcher.read_message().await {
            for line in std::str::from_utf8(&data).unwrap().split('\n') {
                line_reader(line.into()).await;
            }
        }
    });
}

async fn kube_log_reader<F, Fut>(
    namespace: String,
    labels: Option<String>,
    line_reader: F,
) -> Result<()>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &namespace);

    let lp = if let Some(labels) = labels {
        ListParams::default().labels(&labels)
    } else {
        ListParams::default()
    };

    let list = pods.list(&lp).await?;

    for pod in list.iter() {
        println!("{}", pod.name_any());
    }

    let Some(pod) = list.iter().next() else {
        panic!("No pods found");
    };

    let lp = LogParams {
        follow: true,
        tail_lines: Some(0),
        ..Default::default()
    };

    let mut logs = pods.log_stream(&pod.name_any(), &lp).await?.lines();

    while let Some(line) = logs.try_next().await? {
        line_reader(line).await;
    }

    Ok(())
}
