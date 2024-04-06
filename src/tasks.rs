use std::{sync::Arc, time};
use tokio;

use crate::config::AppState;
use crate::error::Result;
use crate::services::whatsminer::models::{Worker, Statistic as TimeStatistic};
use crate::services::whatsminer::types::{Client, Statistic};


async fn collect_statistic(app_state: Arc<AppState>) -> Result<()> {
    log::info!("Start collect_statistic");

    loop {
        let workers = Worker::all(Arc::clone(&app_state.db))
            .await.expect("Error in collect_statistic");

        let mut stats: Vec<Statistic> = Vec::with_capacity(workers.len());

        for worker in workers.iter() {
            let summary = Client::new(worker.host.clone(), worker.port.clone(), worker.name.clone(), false)
                .await?
                .summary()
                .await?;

            stats.push(Statistic { summary , worker: worker.clone() });
        };

        for stat in stats {
            TimeStatistic::create(app_state.db.clone(), stat).await.unwrap();
        };

        tokio::time::sleep(time::Duration::from_secs(60u64)).await;
    };
}


pub async fn register_tasks(app_state: Arc<AppState>) -> Result<(), ()> {
    log::info!("Try to start tasks");

    let copy_state = Arc::clone(&app_state);
    tokio::spawn(collect_statistic(copy_state));

    Ok(())
}