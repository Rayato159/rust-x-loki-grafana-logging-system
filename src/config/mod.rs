use anyhow::Result;
use model::DotEnvyConfig;

pub mod model;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    Ok(DotEnvyConfig {
        loki_url: std::env::var("LOKI_URL").expect("LOKI_URL must be set"),
        loki_job_name: std::env::var("LOKI_JOB_NAME").expect("LOKI_JOB_NAME must be set"),
        loki_service_name: std::env::var("LOKI_SERVICE_NAME")
            .expect("LOKI_SERVICE_NAME must be set"),
    })
}
