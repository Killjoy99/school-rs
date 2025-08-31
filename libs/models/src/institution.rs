use sqlx::{FromRow, types::JsonValue};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub name: String,
    pub code: String,       //unique short code for the institution
    pub address: String,
    pub phone: String,
    pub email: String,
    pub institution_type: InstitutionType,      //critical field
    pub established_date: Option<NaiveDate>,
    pub website: Option<String>,
    pub config: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InstitutionType {
    EarlyChildhoodCenter,
    PrimarySchool,
    SecondarySchool,
    HighSchool,
    VocationalSchool,
    University,             // College or University
    TrainingCenter,
    Other,
}