use common::get;
use std::collections::HashMap;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn fetch_as_markdown(url: String, headers: Vec<bindings::Header>) -> Result<String, String> {
        spin_executor::run(async move {
            let headers: HashMap<String, String> =
                headers.into_iter().map(|h| (h.name, h.value)).collect();
            let response = get(&url, &headers)
                .await
                .map_err(|e| format!("fetch error: {e}"))?;
            let content = String::from_utf8_lossy(response.body()).into_owned();

            // Check content type to determine conversion method
            let content_type = response
                .headers()
                .find(|(name, _)| name.to_lowercase() == "content-type")
                .map(|(_, value)| value.as_bytes())
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();

            if content_type.contains("application/json") {
                Ok(common::json_to_markdown(&content))
            } else {
                Ok(common::html_to_markdown(&content))
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);
