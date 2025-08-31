use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub code: String,       //e.g "user:create", "course:update"
    pub name: String,       //e.g "Create User", "Update Course"
    pub description: String,
    pub category: String,       // e.g "User Management"
    pub is_custom: bool,        // Tracks if it's a builtin or custom permission
    pub created_at: DateTime<Utc>
}

// Struct that represents a row in the permission table