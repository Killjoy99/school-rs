use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    pub id: i32, // ✅ Primary key for this relationship
    pub role_id: i32, // References roles.id (static role)
    pub user_id: Uuid, // References users.id
    pub institution_id: Uuid, // ✅ ESSENTIAL: References institutions.id (multi-tenancy)
    pub assigned_by: Option<Uuid>, // References users.id (for auditing)
    pub assigned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>, // For temporary role assignments
    // For intergrity foreign key to user_institutions
    user_institution_id: Option<Uuid>,
}