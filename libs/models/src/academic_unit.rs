use sqlx::{FromRow, types::JsonValue};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
// This unit can be nested and can represent e.g 1st grade, university department


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AcademicUnit {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub parent_unit_id: Uuid,               // For nested structures (Department->Fauculty | Course->Module)
    pub name: String,
    pub unit_type: AcademicUnitType,
    pub grade_scale_id: Option<Uuid>,
    pub metadata: JsonValue,    //flexible data
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcademicUnitType {
    Campus,             // can represent a cohort
    Fauculty,
    Department,
    School,             // e.g school of engineering
    GradeLevel,         // e.g 1st, 2nd (Primary/Secondary/High School)
    YearGroup,          // e.g Year 1, Year 2 (University/College)
    Program,            // MBA, MSc
    Course,             // Mechanics 101
    Section,            // A specific Section of a course
}
