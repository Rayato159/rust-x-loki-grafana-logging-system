use std::fmt;

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
