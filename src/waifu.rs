use crate::config::Config;
use rand::Rng;

#[derive(serde::Serialize)]
pub struct Waifu {
    pub id: usize,
    pub url: String,
}

pub fn random(config: &Config) -> usize {
    rand::thread_rng().gen_range(0..config.image_amount)
}

pub fn check(id: usize, config: &Config) -> Option<Waifu> {
    if id >= config.image_amount {
        None
    } else {
        Some(Waifu {
            id,
            url: format!("{}/img/seed{:04}.png", config.base_url, id),
        })
    }
}
