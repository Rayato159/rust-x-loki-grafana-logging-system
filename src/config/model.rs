#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub loki_url: String,
    pub loki_job_name: String,
    pub loki_service_name: String,
}
