use serde::{Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct AddUserToInstitutionSchema {
    pub user_id: Uuid,
    pub institution_id: Uuid,
    pub membership_type: Option<String>,
    pub invited_by: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserInstitutionSchema {
    pub membership_type: Option<String>,
    pub is_active: Option<bool>,
    pub left_at: Option<DateTime<Utc>>,
}