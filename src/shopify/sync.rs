use crate::utils::theme::ThemeFile;

use super::{get_assets::ShopifyAsset, Shopify};

impl Shopify {
    pub async fn sync(&self, theme_files: Vec<ThemeFile>) -> Result<(), ()> {
        let actual_assets = self.get_assets().await;
        let mut assets_to_upload: Vec<ThemeFile> = Vec::new();
        let mut assets_to_delete: Vec<ShopifyAsset> = Vec::new();

        // Detect changes from the theme files compared to the actual assets
        for theme_file in theme_files.clone() {
            // Find the asset in the actual assets
            let asset = actual_assets
                .iter()
                .find(|asset| asset.key == theme_file.file_name);

            // If the asset is not found, it means it's a new asset
            if asset.is_none() {
                assets_to_upload.push(theme_file);
            } else {
                // If the asset is found, check if the checksum is different
                let asset = asset.unwrap();
                if let Some(checksum) = &asset.checksum {
                    if checksum != &theme_file.checksum {
                        assets_to_upload.push(theme_file);
                    }
                }
            }
        }

        // Detect files to delete which are not in the theme files
        for asset in actual_assets {
            // Skip files which are in locales folder
            // Skip files which are in templates/*.json
            if asset.key.starts_with("locales/")
                || (asset.key.starts_with("templates/") && asset.key.ends_with(".json"))
                || asset.key == "config/settings_data.json"
            {
                continue;
            }

            let theme_file = theme_files
                .iter()
                .find(|theme_file| theme_file.file_name == asset.key);

            if theme_file.is_none() {
                assets_to_delete.push(asset);
            }
        }

        println!(
            "[{}]: Uploading {} assets",
            self.store,
            assets_to_upload.len()
        );

        // Upload assets
        for (index, asset) in assets_to_upload.iter().enumerate() {
            self.upload_asset(asset).await;
            println!(
                "[{}]: Uploaded {}/{} files",
                self.store,
                index + 1,
                assets_to_upload.len()
            );
        }

        println!("[{}]: The assets have been uploaded", self.store);
        println!(
            "[{}]: Deleting {} assets",
            self.store,
            assets_to_delete.len()
        );

        // Delete assets
        for (index, asset) in assets_to_delete.iter().enumerate() {
            self.delete_asset(asset).await;
            println!(
                "[{}]: Deleted {}/{} files",
                self.store,
                index + 1,
                assets_to_delete.len()
            );
        }

        println!("[{}]: The assets have been deleted", self.store);

        Ok(())
    }
}
