use tera::{Context, Tera};
use actix_web::{error::ErrorInternalServerError, Error, HttpResponse};
use chrono::Local;
use chrono::Datelike;


pub fn render_template(tmpl: &Tera, template_name: &str, mut ctx: Context) -> Result<HttpResponse, Error> {
    // Add common context variables
    ctx.insert("current_year", &Local::now().year());

    let rendered = tmpl.render(template_name, &ctx)
        .map_err(|e| ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}