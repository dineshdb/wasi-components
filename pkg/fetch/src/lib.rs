use common::get;
use std::collections::HashMap;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn fetch(url: String, headers: Vec<bindings::Header>) -> Result<String, String> {
        spin_executor::run(async move {
            let headers: HashMap<String, String> =
                headers.into_iter().map(|h| (h.name, h.value)).collect();
            let response = get(&url, &headers).await.map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(response.body()).to_string())
        })
    }
}

bindings::export!(Component with_types_in bindings);
