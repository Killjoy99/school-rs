use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: String, // "student", "teacher", "admin"
    balance: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub grade: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub teacher: String,
}

#[derive(Serialize)]
pub struct DashboardCard {
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub link: String,
    pub footer: String,
}

#[derive(Serialize)]
pub struct Event {
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub color: String,
}