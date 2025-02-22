use crate::domain::{weapon_entities::NewWeaponDto, weapon_repository::WeaponRepository};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use uuid::Uuid;

pub struct WeaponDB;

impl WeaponDB {
    pub fn new() -> Self {
        Self
    }

    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WeaponRepository for WeaponDB {
    async fn add(&self, new_weapon_dto: NewWeaponDto) -> Result<Uuid> {
        Ok(new_weapon_dto.id)
    }
}
