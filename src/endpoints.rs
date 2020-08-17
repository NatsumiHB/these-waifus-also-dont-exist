use crate::config::Config;
use crate::waifu;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, web, HttpResponse};

mod html_template;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(choose_json)
        .service(random_json)
        .service(choose)
        .service(random);
}

#[get("/")]
async fn random() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_template::get_html(waifu::random(), false))
}

#[get("/get_json")]
async fn random_json(config: Data<Config>) -> Json<waifu::Waifu> {
    let id = waifu::random();

    Json(waifu::Waifu {
        id,
        url: format!("{}/img/seed{:04}.png", config.base_url, id),
    })
}

#[get("/{id}")]
async fn choose(id: web::Path<usize>, config: Data<Config>) -> HttpResponse {
    let id = id.into_inner();

    match waifu::check(id, &config) {
        Some(_waifu) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html_template::get_html(id, true)),
        None => HttpResponse::NotFound()
            .content_type("text/html")
            .body(html_template::NOT_FOUND),
    }
}

#[get("/get_json/{id}")]
async fn choose_json(id: web::Path<usize>, config: Data<Config>) -> Option<Json<waifu::Waifu>> {
    waifu::check(id.into_inner(), &config).map(Json)
}
