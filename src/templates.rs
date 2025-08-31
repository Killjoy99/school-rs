use lazy_static::lazy_static;
use tera::Tera;

pub fn init_tera() -> Tera {
    lazy_static! {
        static ref TERA: Tera = {
            let tera = match Tera::new("templates/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    std::process::exit(1);
                }
            };
            tera
        };
    }
    TERA.clone()
}
