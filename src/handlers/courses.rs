use actix_web::{web, HttpResponse};
use actix_session::Session;
use sqlx::MySqlPool;
use tera::{Tera, Context};

use crate::models::Course;
use crate::utils::render_template;

pub async fn index(tmpl: web::Data<Tera>, pool: web::Data<MySqlPool>, session: Session) -> HttpResponse {
    let mut ctx = Context::new();
    // Get all courses from the database
    let courses = sqlx::query_as::<_, Course>("SELECT * FROM courses")
        .fetch_all(&**pool)
        .await
        .unwrap_or_else(|_| vec![]);

    ctx.insert("title", "Courses");
    ctx.insert("courses", &courses);

    render_template(&tmpl, "courses.html.tera", ctx).expect("Failed to render template")
}