use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInstitution {
    pub id: Uuid,
    pub user_id: Uuid,
    pub institution_id: Uuid,
    pub membership_type: String,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
    pub invited_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
}
