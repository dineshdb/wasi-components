#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn html_to_markdown(html: String) -> String {
        common::html_to_markdown(&html)
    }

    fn json_to_markdown(json: String) -> String {
        common::json_to_markdown(&json)
    }
}

bindings::export!(Component with_types_in bindings);
