use std::sync::Arc;

use ntex::web::{App, HttpServer};
use observability_but_rust::{
    application::weapon_use_case::WeaponUseCase,
    infrastructure::{mock_db::repository::WeaponDB, ntex_http::weapon_routes::weapon_routes},
};
use tracing::info;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

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
    .await
}
