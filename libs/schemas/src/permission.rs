use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePermissionSchema {
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: String,
}