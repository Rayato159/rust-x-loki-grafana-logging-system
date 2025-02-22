use std::sync::Arc;

use ntex::web::{
    self, Responder, ServiceConfig, post, scope,
    types::{Json, State},
};
use tracing::{error, info};

use crate::{
    application::weapon_use_case::WeaponUseCase,
    domain::weapon_models::WeaponAddingModel,
    infrastructure::{
        loki::{ActionResponse, Routes, WeaponRoutes},
        mock_db::repository::WeaponDB,
    },
};

#[post("")]
pub async fn add(
    weapon_use_case: State<Arc<WeaponUseCase<WeaponDB>>>,
    weapon_adding_model: Json<WeaponAddingModel>,
) -> impl Responder {
    match &weapon_use_case.add(weapon_adding_model.into_inner()).await {
        Ok(weapon_id) => {
            let msg = format!("Add weapon succeed: {}", weapon_id);

            info!(
                task = Routes::Weapon(WeaponRoutes::Add).to_string(),
                result = ActionResponse::Succeed(weapon_id.to_string()).to_string(),
                msg,
            );
            web::HttpResponse::Created().body(msg)
        }
        Err(err) => {
            error!(
                task = "tracing_setup",
                result = "success",
                "tracing successfully set up",
            );
            web::HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub fn weapon_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/weapons").service(add));
}
