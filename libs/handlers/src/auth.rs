use actix_session::Session;
use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;
use tera::Tera;
use uuid::Uuid;

use models::User;
use utils::render_template;

pub async fn login_form(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Login");
    ctx.insert("show_footer", &false);
    ctx.insert("show_header", &false);

    render_template(&tmpl, "login.html.tera", ctx).unwrap()
}

// pub async fn login(
//     form: web::Form<(String, String)>, // username, password
//     pool: web::Data<MySqlPool>,
//     session: Session,
// ) -> HttpResponse {
//     let (username, password) = form.into_inner();

//     let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
//         .bind(&username)
//         .fetch_optional(&**pool)
//         .await
//         .unwrap();

//     if let Some(user) = user {
//         if verify(&password, &user.password_hash).unwrap() {
//             session.insert("user_id", user.id).unwrap();
//             session.insert("role", &user.role).unwrap();
//             return HttpResponse::Found().append_header(("Location", "/")).finish();
//         }
//     }

//     HttpResponse::Unauthorized().body("Invalid username or password")
// }

pub async fn register_form(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Register");
    render_template(&tmpl, "register.html.tera", ctx).unwrap()
}

pub async fn register(
    form: web::Form<(String, String, String)>, // username, password, role
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let (username, password, role) = form.into_inner();

    let password_hash = hash(password, DEFAULT_COST).unwrap();

    let _ = sqlx::query("INSERT INTO users (id, username, password_hash, role) VALUES ($1, $2, $3, $4)")
        .bind(Uuid::new_v4())
        .bind(username)
        .bind(password_hash)
        .bind(role)
        .execute(&**pool)
        .await;

    HttpResponse::Found().append_header(("Location", "/login")).finish()
}

pub async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Found().append_header(("Location", "/login")).finish()
}
