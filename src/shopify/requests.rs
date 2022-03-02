use crate::utils::run_safely;

use super::Shopify;

impl Shopify {
    pub fn format_url(&self, endpoint: &str) -> String {
        format!(
            "https://{}.myshopify.com/admin/api/2022-01/themes/{}/{}",
            self.store, self.theme_id, endpoint
        )
    }

    pub async fn get(&self, endpoint: &str) -> String {
        async fn query((url, password): &(String, &String)) -> Result<String, String> {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("X-Shopify-Access-Token", password.parse().unwrap());

            let request = reqwest::Client::new()
                .get(url)
                .headers(headers)
                .send()
                .await;

            if request.is_err() {
                return Err(request.err().unwrap().to_string());
            }

            let response = request.unwrap();

            // If status is 429, wait 5s
            if response.status() == 429 {
                std::thread::sleep(std::time::Duration::from_secs(5));
                return Err(String::from("Too many requests"));
            }

            let body = response.text().await;

            if body.is_err() {
                return Err(body.err().unwrap().to_string());
            }

            Ok(body.unwrap())
        }

        let args = (self.format_url(endpoint), &self.password);
        run_safely::run_async(query, 10, &args).await.unwrap()
    }
    pub async fn put(&self, endpoint: &str, body: String) -> String {
        async fn query(
            (url, password, body): &(String, &String, String),
        ) -> Result<String, String> {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("X-Shopify-Access-Token", password.parse().unwrap());

            let request = reqwest::Client::new()
                .put(url)
                .headers(headers)
                .body(body.clone())
                .send()
                .await;

            if request.is_err() {
                return Err(request.err().unwrap().to_string());
            }

            let response = request.unwrap();

            // If status is 429, wait 5s
            if response.status() == 429 {
                std::thread::sleep(std::time::Duration::from_secs(5));
                return Err(String::from("Too many requests"));
            }

            let body = response.text().await;

            if body.is_err() {
                return Err(body.err().unwrap().to_string());
            }

            Ok(body.unwrap())
        }

        let args = (self.format_url(endpoint), &self.password, body);
        run_safely::run_async(query, 10, &args).await.unwrap()
    }
    pub async fn delete(&self, endpoint: &str) -> String {
        async fn query((url, password): &(String, &String)) -> Result<String, String> {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("X-Shopify-Access-Token", password.parse().unwrap());

            let request = reqwest::Client::new()
                .delete(url)
                .headers(headers)
                .send()
                .await;

            if request.is_err() {
                return Err(request.err().unwrap().to_string());
            }

            let response = request.unwrap();

            // If status is 429, wait 5s
            if response.status() == 429 {
                std::thread::sleep(std::time::Duration::from_secs(5));
                return Err(String::from("Too many requests"));
            }

            let body = response.text().await;

            if body.is_err() {
                return Err(body.err().unwrap().to_string());
            }

            Ok(body.unwrap())
        }

        let args = (self.format_url(endpoint), &self.password);
        run_safely::run_async(query, 10, &args).await.unwrap()
    }
}
