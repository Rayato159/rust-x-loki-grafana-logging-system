use anyhow::Result;
use ntex::web::{App, HttpServer};
use observability_but_rust::{
    application::weapon_use_case::WeaponUseCase,
    config::load,
    infrastructure::{
        loki::loki_tracing_setup, mock_db::repository::WeaponDB,
        ntex_http::weapon_routes::weapon_routes,
    },
};
use std::sync::Arc;
use tracing::info;

#[ntex::main]
async fn main() -> Result<()> {
    let dotenvy_config = load()?;
    let (controller, task) = loki_tracing_setup(&dotenvy_config)?;

    info!(
        task = "tracing_setup",
        result = "success",
        "tracing successfully set up",
    );

    HttpServer::new(move || {
        let weapon_use_case_artifact = {
            let weapon_repository = WeaponDB::new();
            let weapon_use_case = WeaponUseCase::new(Arc::new(weapon_repository));
            Arc::new(weapon_use_case)
        };

        App::new()
            .state(Arc::clone(&weapon_use_case_artifact))
            .configure(weapon_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    info!(
        task = "shutdown",
        result = "success",
        "server successfully shut down",
    );

    controller.shutdown().await;
    task.await.unwrap();

    Ok(())
}
