use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;
use std::fs::read_dir;

mod config;
mod endpoints;
mod waifu;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let image_amount = read_dir("./img").unwrap().count();
    let padding_width = image_amount.to_string().chars().count() - 1;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/img", "./img"))
            .configure(endpoints::configure)
            .data(Config {
                base_url: env::var("BASE_URL")
                    .unwrap_or_else(|_| "https://waifus-are.fun-stuff.xyz".to_owned()),
                image_format: env::var("IMAGE_FORMAT").unwrap_or_else(|_| "webp".to_owned()),
                image_amount,
                default_id: env::var("DEFAULT_ID")
                    .ok()
                    .and_then(|it| it.parse().ok())
                    .unwrap_or(2),
                padding_width,
            })
    })
    .bind("0.0.0.0:5002")?
    .run()
    .await?;

    Ok(())
}
