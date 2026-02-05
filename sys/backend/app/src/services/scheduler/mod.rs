mod jobs;

use std::sync::Arc;

use sqlx::PgPool;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use crate::config::Config;

pub struct ChallengeScheduler {
    scheduler: JobScheduler,
}

impl ChallengeScheduler {
    pub async fn new(pool: Arc<PgPool>, config: Arc<Config>) -> Result<Self, anyhow::Error> {
        let scheduler = JobScheduler::new().await?;

        let cron_schedule = config.challenge_generation.cron_schedule.clone();
        let daily_count = config.challenge_generation.daily_count;

        let job = Job::new_async(cron_schedule.as_str(), move |_uuid, _lock| {
            let pool = pool.clone();
            let config = config.clone();
            Box::pin(async move {
                info!("Starting scheduled challenge generation job");
                match jobs::generate_challenges(&pool, &config, daily_count).await {
                    Ok(count) => info!("Successfully generated {} challenges", count),
                    Err(e) => error!("Failed to generate challenges: {}", e),
                }
            })
        })?;

        scheduler.add(job).await?;

        Ok(Self { scheduler })
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        info!("Starting challenge generation scheduler");
        self.scheduler.start().await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), anyhow::Error> {
        info!("Shutting down challenge generation scheduler");
        self.scheduler.shutdown().await?;
        Ok(())
    }
}
