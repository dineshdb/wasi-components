#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn get_current_time() -> String {
        // Get current UTC time and format it as ISO 8601
        let now = chrono::Utc::now();
        now.to_rfc3339()
    }
}

bindings::export!(Component with_types_in bindings);
