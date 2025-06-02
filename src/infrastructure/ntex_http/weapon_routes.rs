use std::sync::Arc;

use ntex::web::{
    self, Responder, ServiceConfig, post, scope,
    types::{Json, State},
};
use opentelemetry::{
    global,
    trace::{Span, SpanKind, Status, Tracer},
};
use tracing::{error, info};

use crate::{
    application::weapon_use_case::WeaponUseCase,
    domain::weapon_models::WeaponAddingModel,
    infrastructure::{
        mock_db::repository::WeaponDB,
        observability::{ActionResponse, Routes, WeaponRoutes},
    },
};

const WEAPON_ROUTES_TRACER_NAME: &str = "weapon_routes";

#[post("")]
pub async fn add(
    weapon_use_case: State<Arc<WeaponUseCase<WeaponDB>>>,
    weapon_adding_model: Json<WeaponAddingModel>,
) -> impl Responder {
    let tracer = global::tracer(WEAPON_ROUTES_TRACER_NAME);
    let mut span = tracer
        .span_builder("add")
        .with_kind(SpanKind::Server)
        .start(&tracer);

    match &weapon_use_case.add(weapon_adding_model.into_inner()).await {
        Ok(weapon_id) => {
            let msg = format!("Add weapon succeed: {}", weapon_id);

            info!(
                task = Routes::Weapon(WeaponRoutes::Add).to_string(),
                result = ActionResponse::Succeed(weapon_id.to_string()).to_string(),
                msg,
            );

            span.set_status(Status::Ok);

            web::HttpResponse::Created().body(msg)
        }
        Err(err) => {
            let msg = err.to_string();

            error!(
                task = Routes::Weapon(WeaponRoutes::Add).to_string(),
                result = ActionResponse::Failed(msg.clone()).to_string(),
                msg,
            );

            span.set_status(Status::error(msg.clone()));

            web::HttpResponse::InternalServerError().body(msg)
        }
    }
}

pub fn weapon_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/weapons").service(add));
}
