use crate::waifu::Waifu;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use rand::Rng;
use std::env;

mod html_template;
mod waifu;

const IMAGE_AMOUNT: usize = 10000;

#[derive(Clone)]
struct Config {
    base_url: String,
}

fn get_random_waifu() -> usize {
    rand::thread_rng().gen_range(0, IMAGE_AMOUNT)
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/img", "./img"))
            .service(random)
            .service(get_json)
            .service(choose)
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

#[get("/")]
async fn random() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_template::get_html(get_random_waifu(), false))
}

#[get("/get_json")]
async fn get_json(config: Data<Config>) -> Json<Waifu> {
    let id = get_random_waifu();

    Json(Waifu {
        id,
        url: format!("{}/img/seed{:04}.png", config.base_url, id),
    })
}

#[get("/{id}")]
async fn choose(id: web::Path<usize>) -> HttpResponse {
    let id = id.into_inner();

    if id >= IMAGE_AMOUNT {
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(html_template::NOT_FOUND);
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_template::get_html(id, true))
}
