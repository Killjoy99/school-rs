use actix_web::{cookie::Key, web, App, HttpServer};
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::io::Result;

// Redis (for sessions)
use deadpool_redis::{Config as RedisConfig};

// My helper modules
use db::connection::{db_connection_string, redis_connection_string};

mod routes;
mod templates;
mod db;
mod models;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // MySql Database
    let db_url = db_connection_string();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Redis Database
    let redis_url = redis_connection_string();
    let redis_cfg = RedisConfig::from_url(redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("❌ Failed to create Redis pool");

    // Templates
    let tera = templates::init_tera();

    // Session secret
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))     //use the templates
            .app_data(web::Data::new(pool.clone()))     // use the database pool
            .app_data(web::Data::new(redis_pool.clone())) // use the redis pool
            .service(fs::Files::new("/static", "./static").show_files_listing()) // static files
            // Session middleware
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
