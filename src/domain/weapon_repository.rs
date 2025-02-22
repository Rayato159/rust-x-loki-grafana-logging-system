use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::weapon_entities::NewWeaponDto;

#[async_trait]
pub trait WeaponRepository {
    async fn add(&self, new_weapon_dto: NewWeaponDto) -> Result<Uuid>;
}
