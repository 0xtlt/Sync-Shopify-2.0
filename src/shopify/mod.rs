#[derive(Debug, Clone)]
pub struct Shopify {
    pub store: String,
    pub password: String,
    pub theme_id: u64,
}

pub(crate) mod assets_management;
pub(crate) mod get_assets;
pub(crate) mod requests;
pub(crate) mod sync;
