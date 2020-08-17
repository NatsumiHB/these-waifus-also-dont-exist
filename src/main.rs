use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

mod config;
mod endpoints;
mod waifu;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/img", "./img"))
            .configure(endpoints::configure)
            .data(Config {
                base_url: env::var("BASE_URL")
                    .unwrap_or("https://waifus-are.fun-stuff.xyz".to_owned()),
            })
    })
    .bind("0.0.0.0:5002")?
    .run()
    .await?;

    Ok(())
}
