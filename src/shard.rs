use std::sync::Arc;

use serenity::all::ShardManager;
use tokio::time::{Duration, sleep};
use tracing::info;

pub(crate) async fn spawn_latency_metrics(shard_manager: Arc<ShardManager>, seconds: u64) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(seconds)).await;
            let shard_runners = shard_manager.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                info!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });
}

pub(crate) async fn spawn_shard_manager_shutdown(shard_manager: Arc<ShardManager>) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });
}
