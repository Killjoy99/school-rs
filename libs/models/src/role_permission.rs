use sqlx::FromRow;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermission {
    pub id: i32,
    pub role_id: i32, // References roles.id
    pub permission_id: i32, // References permissions.id
    pub assigned_by: Option<Uuid>, // References users.id (for auditing)
    pub assigned_at: DateTime<Utc>,
}
