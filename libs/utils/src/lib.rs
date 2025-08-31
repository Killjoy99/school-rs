use lazy_static::lazy_static;
use tera::{Context, Tera};
use actix_web::{error::ErrorInternalServerError, Error, HttpResponse};
use chrono::{Local, Datelike};
use std::path::PathBuf;

pub fn init_tera() -> Tera {
    lazy_static! {
        static ref TERA: Tera = {
            // Get the project root directory
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .into_owned());

            // Go up one level to reach the project root
            let workspace_root = PathBuf::from(&manifest_dir)
                .parent()
                .expect("Failed to get parent directory")
                .to_path_buf();
            
            // Build the absolute path to templates
            let template_path = workspace_root
                .join("assets")
                .join("templates")
                .join("**")
                .join("*");
            
            let template_pattern = template_path.to_string_lossy();
            println!("Loading templates from: {}", template_pattern);
            
            let tera = match Tera::new(&template_pattern) {
                Ok(t) => {
                    println!("Successfully loaded templates: {:?}", t.get_template_names().collect::<Vec<_>>());
                    t
                },
                Err(e) => {
                    eprintln!("Template parsing error: {}", e);
                    // Debug using the original &str without move
                    let check_path = PathBuf::from(&manifest_dir).join("assets").join("templates");
                    if check_path.exists() {
                        eprintln!("Directory exists: {:?}", check_path);
                        if let Ok(entries) = std::fs::read_dir(&check_path) {
                            eprintln!("Files in directory:");
                            for entry in entries.flatten() {
                                eprintln!("  - {}", entry.file_name().to_string_lossy());
                            }
                        }
                    } else {
                        eprintln!("Directory does NOT exist: {:?}", check_path);
                    }
                    std::process::exit(1);
                }
            };
            tera
        };
    }
    TERA.clone()
}

pub fn render_template(tmpl: &Tera, template_name: &str, mut ctx: Context) -> Result<HttpResponse, Error> {
    // Add common context variables
    ctx.insert("current_year", &Local::now().year());

    let rendered = tmpl.render(template_name, &ctx)
        .map_err(ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}