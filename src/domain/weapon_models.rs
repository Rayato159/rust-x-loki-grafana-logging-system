use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::weapon_entities::NewWeaponDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAddingModel {
    pub name: String,
    pub damage: i32,
}

impl WeaponAddingModel {
    pub fn to_dto(&self) -> NewWeaponDto {
        NewWeaponDto {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            damage: self.damage,
        }
    }
}
