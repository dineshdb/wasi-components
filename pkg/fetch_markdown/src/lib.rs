// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use spin_sdk::http::{Request, Response, send};

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn fetch_as_markdown(url: String, headers: Vec<bindings::Header>) -> Result<String, String> {
        spin_executor::run(async move {
            // Fetch the HTML content
            let mut request = Request::get(url);
            for header in headers {
                request.header(header.name.clone().as_str(), header.value.as_str());
            }

            let response: Response = send(request.build()).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!("Request failed with status code: {status}"));
            }
            let body = response.body();
            let content = String::from_utf8_lossy(body).into_owned();

            // Check content type to determine conversion method
            let content_type = response
                .headers()
                .find(|(name, _)| name.to_lowercase() == "content-type")
                .map(|(_, value)| value.as_bytes())
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();

            if content_type.contains("application/json") {
                Ok(json_to_markdown(&content))
            } else {
                Ok(html_to_markdown(&content))
            }
        })
    }
}

fn html_to_markdown(html: &str) -> String {
    let mut markdown = String::new();
    let fragment = scraper::Html::parse_fragment(html);
    let text_selector = scraper::Selector::parse("h1, h2, h3, h4, h5, h6, p, a, div, ul, ol, li, pre, code, blockquote, img, table, tr, th, td").unwrap();

    for element in fragment.select(&text_selector) {
        let tag_name = element.value().name();
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if text.is_empty() && !is_special_element(&element) {
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
                    let link_text = if text.is_empty() { href } else { &text };
                    markdown.push_str(&format!("[{link_text}]({href})"));
                } else {
                    markdown.push_str(&text.to_string());
                }
            }
            "ul" => markdown.push('\n'),
            "ol" => markdown.push('\n'),
            "li" => {
                // Check if this is in an ordered list
                let parent = element.parent().and_then(|n| n.value().as_element());
                let is_ordered = parent.is_some_and(|p| p.name() == "ol");
                if is_ordered {
                    markdown.push_str(&format!("1. {text}\n"));
                } else {
                    markdown.push_str(&format!("- {text}\n"));
                }
            }
            "pre" => {
                let code_content = element.text().collect::<String>();
                markdown.push_str(&format!("```\n{code_content}\n```\n\n"));
            }
            "code" => {
                // Only handle inline code (not inside pre)
                if element
                    .parent()
                    .and_then(|n| n.value().as_element())
                    .is_none_or(|p| p.name() != "pre")
                {
                    markdown.push_str(&format!("`{text}`"));
                }
            }
            "blockquote" => {
                let lines: Vec<&str> = text.lines().collect();
                for line in lines {
                    if !line.trim().is_empty() {
                        markdown.push_str(&format!("> {line}\n"));
                    }
                }
                markdown.push('\n');
            }
            "img" => {
                if let Some(src) = element.value().attr("src") {
                    let alt = element.value().attr("alt").unwrap_or("image");
                    markdown.push_str(&format!("![{alt}]({src})\n\n"));
                }
            }
            "table" => markdown.push('\n'),
            "tr" => markdown.push('\n'),
            "th" => {
                markdown.push_str(&format!("| {text} "));
            }
            "td" => {
                markdown.push_str(&format!("| {text} "));
            }
            _ => markdown.push_str(&format!("{text}\n\n")),
        }
    }

    // Process tables to add proper formatting
    markdown = process_tables(&markdown);

    markdown.trim().to_string()
}

fn is_special_element(element: &scraper::ElementRef) -> bool {
    let tag_name = element.value().name();
    matches!(tag_name, "img" | "pre" | "table" | "tr" | "th" | "td")
}

fn process_tables(markdown: &str) -> String {
    let mut result = String::new();
    let mut in_table = false;
    let mut table_lines: Vec<String> = Vec::new();

    for line in markdown.lines() {
        if line.contains("|") && (line.trim().starts_with("|") || line.contains(" | ")) {
            if !in_table {
                in_table = true;
                table_lines.clear();
            }
            table_lines.push(line.to_string());
        } else {
            if in_table {
                // Process the collected table lines
                if !table_lines.is_empty() {
                    // Add header separator
                    if table_lines.len() > 1 {
                        let header_count = table_lines[0].matches('|').count();
                        let separator = "|".repeat(header_count).replace("|", "| --- ");
                        table_lines.insert(1, separator.trim_end().to_string());
                    }

                    for table_line in &table_lines {
                        result.push_str(table_line);
                        result.push('\n');
                    }
                    result.push('\n');
                }
                in_table = false;
                table_lines.clear();
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    // Handle table at the end of content
    if in_table && !table_lines.is_empty() {
        if table_lines.len() > 1 {
            let header_count = table_lines[0].matches('|').count();
            let separator = "|".repeat(header_count).replace("|", "| --- ");
            table_lines.insert(1, separator.trim_end().to_string());
        }

        for table_line in &table_lines {
            result.push_str(table_line);
            result.push('\n');
        }
        result.push('\n');
    }

    result
}

fn json_to_markdown(json: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    json_value_to_markdown(&value)
}

fn json_value_to_markdown(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
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
        serde_json::Value::Array(arr) => {
            let mut markdown = String::new();
            for (i, val) in arr.iter().enumerate() {
                markdown.push_str(&format!("1. {}\n", json_value_to_markdown(val)));
                if i < arr.len() - 1 {
                    markdown.push('\n');
                }
            }
            markdown
        }
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
    }
}

bindings::export!(Component with_types_in bindings);
