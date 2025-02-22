use crate::config::model::DotEnvyConfig;
use anyhow::Result;
use ntex::rt::JoinHandle;
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
        .label("service_name", &dotenvy_config.loki_service_name)?
        .extra_field("pid", format!("{}", process::id()))?
        .build_controller_url(Url::parse(&dotenvy_config.loki_url).unwrap())?;

    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(layer)
        .with(Layer::new())
        .init();

    Ok((controller, tokio::spawn(task)))
}
