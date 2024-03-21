use std::{sync::Arc, time};
use tokio;

use crate::config::AppState;
use crate::services::whatsminer::models::{Worker, Statistic as TimeStatistic};
use crate::services::whatsminer::types::{Client, Statistic};


async fn collect_statistic(app_state: Arc<AppState>) {
    loop {
        log::info!("Try to start collect_statistic");

        let workers = Worker::all(Arc::clone(&app_state.db))
            .await.expect("Error in collect_statistic");

        let stats: Vec<Statistic> = workers
            .iter()
            .map(|el| {
                let summary = Client::new(el.host.clone(), el.port.clone(), el.name.clone(), false).unwrap().summary();
                Statistic { summary, worker: el.clone() }
            })
            .collect();

        for stat in stats {
            TimeStatistic::create(Arc::clone(&app_state.db), Arc::new(stat)).await.unwrap();
        }

        tokio::time::sleep(time::Duration::from_secs(5u64)).await;
    };
}


pub async fn register_tasks(app_state: Arc<AppState>) -> Result<(), ()> {

    log::info!("Try to start tasks");

    let copy_state = Arc::clone(&app_state);
    tokio::spawn(collect_statistic(copy_state));

    Ok(())
}