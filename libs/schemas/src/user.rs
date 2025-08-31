use serde::{Deserialize};
use chrono::NaiveDate;

// DTO for creating a user
#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
}