use crate::config::{Config, IMAGE_AMOUNT};
use rand::Rng;

#[derive(serde::Serialize)]
pub struct Waifu {
    pub id: usize,
    pub url: String,
}

pub fn random() -> usize {
    rand::thread_rng().gen_range(0, IMAGE_AMOUNT)
}
pub fn check(id: usize, config: &Config) -> Option<Waifu> {
    if id >= IMAGE_AMOUNT {
        None
    } else {
        Some(Waifu {
            id,
            url: format!("{}/img/seed{:04}.png", config.base_url, id),
        })
    }
}
