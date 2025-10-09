use common::post_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

#[derive(Serialize)]
struct SearchRequest {
    query: String,
    max_results: u32,
}

#[derive(Serialize)]
struct WebFetchRequest {
    url: String,
}

#[derive(Deserialize)]
struct SearchResult {
    title: String,
    url: String,
    content: String,
}

#[derive(Deserialize)]
struct FetchResult {
    title: String,
    content: String,
    links: Vec<String>,
}

#[derive(Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

struct Component;

impl Component {
    /// Get API key from environment variable
    fn get_api_key() -> Result<String, String> {
        std::env::var("OLLAMA_API_KEY")
            .map_err(|_| "OLLAMA_API_KEY environment variable not set".to_string())
    }

    /// Create common headers for Ollama API requests
    fn create_headers(api_key: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers
    }

    /// Make a POST request to Ollama API and parse JSON response
    async fn make_ollama_request<T: for<'de> Deserialize<'de>>(
        url: &str,
        request_body: &impl Serialize,
    ) -> Result<T, String> {
        let api_key = Self::get_api_key()?;
        let headers = Self::create_headers(&api_key);
        let body = serde_json::to_vec(request_body)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let response = post_json(url, &headers, body)
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let response_body = String::from_utf8_lossy(response.body()).into_owned();
        serde_json::from_str(&response_body).map_err(|e| format!("Failed to parse response: {}", e))
    }
}

impl Guest for Component {
    fn search(query: String, max_results: u32) -> Result<Vec<bindings::SearchResult>, String> {
        spin_executor::run(async move {
            let url = "https://ollama.com/api/web_search";
            let request = SearchRequest { query, max_results };

            let search_response: SearchResponse =
                Component::make_ollama_request(url, &request).await?;

            let results = search_response
                .results
                .into_iter()
                .map(|r| bindings::SearchResult {
                    title: r.title,
                    url: r.url,
                    content: r.content,
                })
                .collect();

            Ok(results)
        })
    }

    fn fetch(url: String) -> Result<bindings::FetchResult, String> {
        spin_executor::run(async move {
            let api_url = "https://ollama.com/api/web_fetch";
            let request = WebFetchRequest { url };

            let fetch_response: FetchResult =
                Component::make_ollama_request(api_url, &request).await?;

            Ok(bindings::FetchResult {
                title: fetch_response.title,
                content: fetch_response.content,
                links: fetch_response.links,
            })
        })
    }
}

bindings::export!(Component with_types_in bindings);
