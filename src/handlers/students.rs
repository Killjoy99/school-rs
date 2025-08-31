use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use tera::{Tera, Context};
use actix_session::Session;

use crate::models::Student;
use crate::utils::render_template;

pub async fn list_students(tmpl: web::Data<Tera>, pool: web::Data<MySqlPool>, session: Session) -> HttpResponse {
    let mut ctx = Context::new();
    // Get all students from the database
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(&**pool)
        .await
        .unwrap_or_else(|_| vec![]);

    ctx.insert("title", "Students");
    ctx.insert("students", &students);

    render_template(&tmpl, "students.html.tera", ctx).expect("Failed to render template")
}