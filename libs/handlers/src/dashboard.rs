use tera::{Tera, Context};
use actix_web::{web, HttpResponse};
use actix_session::Session;
use sqlx::MySqlPool;

// Use the render util to insert date into the context
use utils::render_template;

// Import the card model to dynamically build dashboard cards
use models::{DashboardCard, Event};

// To fetch students
use models::{Student, Course};

pub async fn dashboard(tmpl: web::Data<Tera>, pool: web::Data<MySqlPool>, session: Session) -> HttpResponse {
    let mut ctx = Context::new();
    
    // dummy account balance
    let balance = 150.75;

    let cards = vec![
        DashboardCard {
            title: "Muleya Shylet".into(),
            subtitle: "PROFILE".into(),
            icon: "fas fa-user text-primary".into(),
            link: "/profile".into(),
            footer: "Profile Information →".into(),
        },
        DashboardCard {
            title: format!("Balance: {:.2}", balance),
            subtitle: if balance > 0.0 { "Outstanding" } else { "Cleared" }.into(),
            icon: if balance > 0.0 {
                "fas fa-credit-card text-danger".into()
            } else {
                "fas fa-credit-card text-success".into()
            },
            footer: "Make Payment →".into(),
            link: "/payments".into(),
        },
        DashboardCard {
            title: "Registration".into(),
            subtitle: "In Progress".into(),
            icon: "fas fa-book text-warning".into(),
            footer: "Continue Registration →".into(),
            link: "/registration".into(),
        },
    ];


    // Create some sample events
    let events = vec![
        Event {
            title: "Normal Registration".into(),
            start: "2025-08-29".into(),
            end: Some("2025-09-05".into()),
            color: "#00f2ff".into(),
        },
        Event {
            title: "Late Registration".into(),
            start: "2025-09-06".into(),
            end: Some("2025-09-10".into()),
            color: "#ff8c00".into(),
        },
        Event {
            title: "Special Registration".into(),
            start: "2025-09-11".into(),
            end: Some("2025-09-15".into()),
            color: "#ff00d6".into(),
        }
    ];
    // Populate Context
    ctx.insert("events", &events);
    ctx.insert("cards", &cards);

    render_template(&tmpl, "dashboard.html.tera", ctx).expect("Failed to render template")
}