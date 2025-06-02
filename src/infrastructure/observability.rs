use crate::config::model::DotEnvyConfig;
use anyhow::Result;
use ntex::rt::JoinHandle;
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use std::fmt;
use std::process;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[derive(Debug, PartialEq)]
pub enum Routes {
    Weapon(WeaponRoutes),
}

impl fmt::Display for Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Routes::Weapon(route) => write!(f, "Weapon: {}", route),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WeaponRoutes {
    Add,
}

impl fmt::Display for WeaponRoutes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeaponRoutes::Add => write!(f, "Add"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionResponse {
    Succeed(String),
    Failed(String),
}

impl fmt::Display for ActionResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionResponse::Succeed(msg) => write!(f, "Succeed: {}", msg),
            ActionResponse::Failed(err) => write!(f, "Failed: {}", err),
        }
    }
}

pub fn loki_tracing_setup(
    dotenvy_config: &DotEnvyConfig,
) -> Result<(tracing_loki::BackgroundTaskController, JoinHandle<()>)> {
    let (layer, controller, task) = tracing_loki::builder()
        .label("job", &dotenvy_config.loki_job_name)?
        .label("service_name", &dotenvy_config.service_name)?
        .extra_field("pid", format!("{}", process::id()))?
        .build_controller_url(Url::parse(&dotenvy_config.loki_url).unwrap())?;

    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(layer)
        .with(Layer::new())
        .init();

    Ok((controller, tokio::spawn(task)))
}

pub fn init_otel_tracer(dotenvy_config: &DotEnvyConfig) -> Result<SdkTracerProvider> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(&dotenvy_config.otel_collector_url)
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(dotenvy_config.service_name.clone())
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    Ok(provider)
}
