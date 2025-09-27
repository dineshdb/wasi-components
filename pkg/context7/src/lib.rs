use serde_json::Value as JsonValue;
use spin_sdk::http::{Request, Response, send};
use urlencoding::encode;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

const CONTEXT7_API_BASE_URL: &str = "https://context7.com/api";
impl Guest for Component {
    fn c7_resolve_library_id(library_name_as_query: String) -> Result<String, String> {
        spin_executor::run(async move {
            let encoded_query = encode(&library_name_as_query);
            let url = format!("{CONTEXT7_API_BASE_URL}/v1/search?query={encoded_query}",);
            let request = Request::get(url);
            let response: Response = send(request).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!("Request failed with status code: {status}"));
            }
            let body_str = String::from_utf8_lossy(response.body());

            let Some(content_type) = response.header("content-type").and_then(|v| v.as_str())
            else {
                return Ok(format!(
                    "Missing content type in response. Body: {}",
                    body_str
                ));
            };

            if !content_type.contains("application/json") {
                return Ok(format!(
                    "Unexpected content type: {content_type}. Body: {body_str}",
                ));
            }
            let Ok(json) = serde_json::from_str::<JsonValue>(&body_str) else {
                return Ok(format!(
                    "Failed to parse API response JSON. Body: {body_str}",
                ));
            };

            let Some(results_node) = json.get("results") else {
                return Ok(format!(
                    "API response did not contain a 'results' field as expected. Body: {body_str}",
                ));
            };

            let Some(results_array) = results_node.as_array() else {
                return Ok(format!(
                    "API response 'results' field was not an array as expected. Body: {body_str}",
                ));
            };

            if results_array.is_empty() {
                return Ok("No libraries found matching your query.".to_string());
            }

            let mut results_text_parts = Vec::new();

            for result_item in results_array {
                let mut item_details = Vec::new();

                let title = result_item
                    .get("title")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("N/A");
                item_details.push(format!("- Title: {}", title));

                let id = result_item
                    .get("id")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("N/A");
                item_details.push(format!("- Context7-compatible library ID: {}", id));

                let description = result_item
                    .get("description")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("N/A");
                item_details.push(format!("- Description: {}", description));

                if let Some(v) = result_item
                    .get("totalSnippets")
                    .and_then(JsonValue::as_i64)
                    .filter(|&v| v >= 0)
                {
                    item_details.push(format!("- Code Snippets: {}", v))
                }

                if let Some(v) = result_item
                    .get("stars")
                    .and_then(JsonValue::as_i64)
                    .filter(|&v| v >= 0)
                {
                    item_details.push(format!("- GitHub Stars: {}", v))
                }

                results_text_parts.push(item_details.join("\n"));
            }

            let header = "Available Libraries (top matches):\n\nEach result includes information like:\n- Title: Library or package name\n- Context7-compatible library ID: Identifier (format: /org/repo)\n- Description: Short summary\n- Code Snippets: Number of available code examples (if available)\n- GitHub Stars: Popularity indicator (if available)\n\nFor best results, select libraries based on name match, popularity (stars), snippet coverage, and relevance to your use case.\n\n---\n";
            let final_text = format!("{}{}", header, results_text_parts.join("\n\n"));

            Ok(final_text)
        })
    }
}

bindings::export!(Component with_types_in bindings);
