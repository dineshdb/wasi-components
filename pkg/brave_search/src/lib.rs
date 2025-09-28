// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use spin_sdk::http::{Request, Response, send};

#[allow(warnings)]
mod bindings;
mod brave;

use bindings::Guest;

use crate::brave::BraveResponse;

struct Component;

impl Guest for Component {
    fn search(params: bindings::SearchParams) -> Result<bindings::SearchResponse, String> {
        spin_executor::run(async move {
            // Get API key from environment variable
            let Ok(api_key) = std::env::var("BRAVE_SEARCH_API_KEY") else {
                println!("BRAVE_SEARCH_API_KEY env not set");
                return Err("BRAVE_SEARCH_API_KEY environment variable not set".to_string());
            };

            // Build the Brave Search API URL
            let base_url = "https://api.search.brave.com/res/v1/web/search";
            let mut url = format!("{}?q={}", base_url, urlencoding::encode(&params.query));

            // Add optional parameters
            if params.limit > 0 {
                url.push_str(&format!("&count={}", params.limit.min(20))); // Max 20 results
            }
            if !params.country.is_empty() {
                url.push_str(&format!("&country={}", params.country));
            }
            if !params.language.is_empty() {
                url.push_str(&format!("&search_lang={}", params.language));
            }
            if !params.safe_search.is_empty() {
                url.push_str(&format!("&safesearch={}", params.safe_search));
            }
            if params.include_text {
                url.push_str("&text_decorations=true&result_filter=web");
            }

            // Create HTTP request
            let mut request = Request::get(&url);
            request.header("Accept", "application/json");
            request.header("X-Subscription-Token", &api_key);

            // Send request
            let response: Response = send(request.build()).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!(
                    "Brave Search API request failed with status code: {status}"
                ));
            }

            // Parse response
            let body = String::from_utf8_lossy(response.body());
            let brave_response: BraveResponse = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse Brave Search response: {}", e))?;

            // Convert to our response format
            let results: Vec<bindings::SearchResult> = brave_response
                .web
                .results
                .into_iter()
                .map(|result| {
                    let description = result.description;
                    bindings::SearchResult {
                        title: result.title,
                        url: result.url,
                        description: description.clone(),
                        text: Some(description),
                    }
                })
                .collect();

            Ok(bindings::SearchResponse {
                total_results: results.len() as u32,
                results,
                query: params.query,
            })
        })
    }
}

bindings::export!(Component with_types_in bindings);
