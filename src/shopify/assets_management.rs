use serde_json::json;

use crate::utils::theme::ThemeFile;

use super::{get_assets::ShopifyAsset, Shopify};

impl Shopify {
    pub async fn upload_asset(&self, asset: &ThemeFile) {
        let data = serde_json::to_string_pretty(&json!({
            "asset": json!({
              "key": asset.file_name,
              "value": asset.content,
            })
        }))
        .unwrap();

        self.put("assets.json", data).await;
    }
    pub async fn delete_asset(&self, asset: &ShopifyAsset) {
        self.delete(&format!("assets.json?asset[key]={}", asset.key))
            .await;
    }
}
