// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#[allow(warnings)]
mod bindings;

use bindings::Guest;
use serde_json::Value;

struct Component;

impl Guest for Component {
    fn html_to_markdown(html: String) -> String {
        let mut markdown = String::new();
        let fragment = scraper::Html::parse_fragment(&html);
        let text_selector = scraper::Selector::parse("h1, h2, h3, h4, h5, h6, p, a, div").unwrap();

        for element in fragment.select(&text_selector) {
            let tag_name = element.value().name();
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if text.is_empty() {
                continue;
            }

            match tag_name {
                "h1" => markdown.push_str(&format!("# {text}\n\n")),
                "h2" => markdown.push_str(&format!("## {text}\n\n")),
                "h3" => markdown.push_str(&format!("### {text}\n\n")),
                "h4" => markdown.push_str(&format!("#### {text}\n\n")),
                "h5" => markdown.push_str(&format!("##### {text}\n\n")),
                "h6" => markdown.push_str(&format!("###### {text}\n\n")),
                "p" => markdown.push_str(&format!("{text}\n\n")),
                "a" => {
                    if let Some(href) = element.value().attr("href") {
                        markdown.push_str(&format!("[{text}]({href})\n\n"));
                    } else {
                        markdown.push_str(&format!("{text}\n\n"));
                    }
                }
                _ => markdown.push_str(&format!("{text}\n\n")),
            }
        }

        markdown.trim().to_string()
    }

    fn json_to_markdown(json: String) -> String {
        let value: Value = serde_json::from_str(&json).unwrap_or_default();
        json_value_to_markdown(&value)
    }
}

fn json_value_to_markdown(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut markdown = String::new();
            for (key, val) in map {
                markdown.push_str(&format!(
                    "### {}\n\n{}\n\n",
                    key,
                    json_value_to_markdown(val)
                ));
            }
            markdown
        }
        Value::Array(arr) => {
            let mut markdown = String::new();
            for (i, val) in arr.iter().enumerate() {
                markdown.push_str(&format!("1. {}\n", json_value_to_markdown(val)));
                if i < arr.len() - 1 {
                    markdown.push('\n');
                }
            }
            markdown
        }
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}

bindings::export!(Component with_types_in bindings);
