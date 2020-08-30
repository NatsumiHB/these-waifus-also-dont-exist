#[derive(Clone)]
pub struct Config {
    pub base_url: String,
    pub image_format: String,
    pub image_amount: usize,
    pub default_id: usize,
    pub padding_width: usize,
}
