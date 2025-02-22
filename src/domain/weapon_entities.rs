use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewWeaponDto {
    pub id: Uuid,
    pub name: String,
    pub damage: i32,
}
