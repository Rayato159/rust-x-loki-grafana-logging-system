#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub loki_url: String,
    pub loki_job_name: String,
    pub service_name: String,
    pub otel_collector_url: String,
}
