use actix_web::web;

use handlers::{auth, dashboard, students, courses, dummy};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(dashboard::dashboard)))
       .service(web::resource("/students").route(web::get().to(students::list_students)))
       .service(web::resource("/courses").route(web::get().to(courses::index)))
       // Auth
       .service(web::resource("/login").route(web::get().to(auth::login_form)).route(web::post().to(auth::login)))
       .service(web::resource("/register").route(web::get().to(auth::register_form)).route(web::post().to(auth::register)))
       .service(web::resource("/logout").route(web::post().to(auth::logout)))
       // Dummy route for testing
         .service(web::resource("/dummy").route(web::get().to(dummy::index)));

}
