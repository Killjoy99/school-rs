use serde::{Serialize};


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