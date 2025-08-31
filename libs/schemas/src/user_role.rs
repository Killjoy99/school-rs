use serde::{Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// DTO for assigning a role to a user
#[derive(Debug, Deserialize)]
pub struct AssignUserRoleSchema {
    pub role_id: i32,
    pub user_id: Uuid,
    pub institution_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
}