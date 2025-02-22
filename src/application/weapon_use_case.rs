use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{weapon_models::WeaponAddingModel, weapon_repository::WeaponRepository};

pub struct WeaponUseCase<T>
where
    T: WeaponRepository + Send + Sync + 'static,
{
    weapon_repository: Arc<T>,
}

impl<T> WeaponUseCase<T>
where
    T: WeaponRepository + Send + Sync,
{
    pub fn new(weapon_repository: Arc<T>) -> Self {
        Self { weapon_repository }
    }

    pub async fn add(&self, weapon_adding_model: WeaponAddingModel) -> Result<Uuid> {
        let new_weapon_dto = weapon_adding_model.to_dto();
        self.weapon_repository.add(new_weapon_dto).await
    }
}
