use actix_web::{web, HttpResponse};
use actix_session::Session;
use sqlx::MySqlPool;
use tera::{Tera, Context};

use utils::render_template;

pub async fn index(tmpl: web::Data<Tera>, pool: web::Data<MySqlPool>, session: Session) -> HttpResponse {
    let mut ctx = Context::new();
    // Get all courses from the database

    render_template(&tmpl, "course-details.html.tera", ctx).expect("Failed to render template")
}