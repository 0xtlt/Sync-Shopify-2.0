use serde_derive::Deserialize;
use serde_json::Value;

use super::Shopify;

#[derive(Debug, Deserialize)]
pub struct ShopifyAsset {
    pub key: String,
    pub public_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub content_type: String,
    pub size: i32,
    pub checksum: Option<String>,
    pub theme_id: u64,
}

impl Shopify {
    pub async fn get_assets(&self) -> Vec<ShopifyAsset> {
        let str_result = self.get("assets.json").await;
        // Deserialize the result
        let request_result: Value = serde_json::from_str(&str_result).unwrap();

        let assets: Vec<ShopifyAsset> =
            serde_json::from_value(request_result["assets"].clone()).unwrap();
        assets
    }
}
