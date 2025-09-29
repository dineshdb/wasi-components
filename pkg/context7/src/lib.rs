use serde_json::Value as JsonValue;
use spin_sdk::http::{Request, Response, send};
use urlencoding::encode;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

const BASE_URL: &str = "https://context7.com/api";

// Helper function to create a request with redirect following
fn create_request(url: &str) -> Request {
    Request::get(url)
        .header("User-Agent", "Context7-MCP-Server/1.0")
        .header("Accept", "application/json")
        .header("X-Context7-Source", "mcp-server")
        .build()
}
impl Guest for Component {
    fn c7_resolve_library_id(library_name_as_query: String) -> Result<String, String> {
        spin_executor::run(async move {
            let encoded_query = encode(&library_name_as_query);
            let url = format!("{BASE_URL}/v1/search?query={encoded_query}",);
            let request = create_request(&url);
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

    fn c7_get_library_docs(
        context7_compatible_library_id: String,
        topic: String,
        tokens: u32,
    ) -> Result<String, String> {
        spin_executor::run(async move {
            let mut id_for_path = context7_compatible_library_id.clone();
            let mut folders_value_opt: Option<String> = None;

            if let Some(idx) = context7_compatible_library_id.rfind("?folders=") {
                let (id_part, folders_part_with_query) =
                    context7_compatible_library_id.split_at(idx);
                id_for_path = id_part.to_string();
                folders_value_opt = Some(
                    folders_part_with_query
                        .trim_start_matches("?folders=")
                        .to_string(),
                );
            }

            let mut query_params_vec = vec![format!(
                "context7CompatibleLibraryID={}",
                encode(&context7_compatible_library_id) // Use the original, full ID string for this query parameter
            )];

            if let Some(folders_val) = &folders_value_opt
                && !folders_val.is_empty()
            {
                query_params_vec.push(format!("folders={}", encode(folders_val)));
            }

            if !topic.is_empty() {
                query_params_vec.push(format!("topic={}", encode(&topic)));
            }

            if tokens > 0 {
                query_params_vec.push(format!("tokens={}", tokens));
            }

            let query_params = query_params_vec.join("&");
            let url = format!("{BASE_URL}/v1{id_for_path}?{query_params}",);
            let req = create_request(&url);

            match send::<_, spin_sdk::http::Response>(req).await {
                Ok(response) => {
                    let status = response.status();
                    let body_str = String::from_utf8_lossy(response.body()).to_string();
                    if (200..300).contains(status) {
                        Ok(body_str)
                    } else {
                        Err(format!(
                            "API request for docs (URL: {url}) failed with status {status}: {body_str}",
                        ))
                    }
                }
                Err(e) => Err(format!("HTTP request for docs failed: {e}, URL: {url}")),
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);
