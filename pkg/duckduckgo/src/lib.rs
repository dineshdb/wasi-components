use common::get;
use regex::Regex;
use std::collections::HashMap;
use urlencoding::decode;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

lazy_static::lazy_static! {
    static ref RESULT_REGEX: Regex = Regex::new(r#"<tr>.*?</tr>"#).unwrap();
    static ref LINK_REGEX: Regex = Regex::new(r#"<a[^>]*class='result-link'[^>]*href="([^"]*)"[^>]*>(.*?)</a>"#).unwrap();
    static ref SNIPPET_REGEX: Regex = Regex::new(r#"<td[^>]*class="result-snippet"[^>]*>(.*?)</td>"#).unwrap();
    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref TEXT_REGEX: Regex = Regex::new(r#"<[^>]*>([^<]*)<[^>]*>"#).unwrap();
    static ref UDDG_REGEX: Regex = Regex::new(r"uddg=([^&]*)").unwrap();
    static ref ALT_LINK_REGEX: Regex = Regex::new(r#"<a[^>]*href="([^"]*)"[^>]*>(.*?)</a>"#).unwrap();
}

impl Guest for Component {
    fn search(query: String) -> Result<bindings::MarkdownResponse, String> {
        spin_executor::run(async move {
            let json = self::search_ddg(query.clone()).await?;
            let markdown = convert_results_to_markdown(&json.results, &json.query);
            Ok(bindings::MarkdownResponse {
                query: json.query,
                markdown,
            })
        })
    }

    fn search_json(query: String) -> Result<bindings::SearchResponse, String> {
        spin_executor::run(async move { self::search_ddg(query).await })
    }
}

/// Internal function to get JSON search results
async fn search_ddg(query: String) -> Result<bindings::SearchResponse, String> {
    let base_url = "https://lite.duckduckgo.com/lite/";
    let url = format!("{}?q={}", base_url, urlencoding::encode(&query));

    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string());

    let response = get(&url, &headers)
        .await
        .map_err(|e| format!("Failed to fetch DuckDuckGo search results: {}", e))?;

    let status = response.status();
    if !(200..300).contains(status) {
        return Err(format!(
            "DuckDuckGo search failed with status code: {status}"
        ));
    }

    let html = String::from_utf8_lossy(response.body()).into_owned();
    let results = parse_duckduckgo_results(&html)?;
    Ok(bindings::SearchResponse {
        results,
        query: query.clone(),
    })
}

/// Convert JSON search results to markdown format
fn convert_results_to_markdown(results: &[bindings::SearchResult], query: &str) -> String {
    let mut markdown = format!("# Results for \"{}\"\n\n", query);

    if results.is_empty() {
        markdown.push_str("No results found.\n");
        return markdown;
    }

    for (index, result) in results.iter().enumerate() {
        markdown.push_str(&format!("{}. {}\n", index + 1, result.title));
        markdown.push_str(&format!("   **URL:** {}\n", result.url));
        if !result.description.is_empty() && result.description != "No description available" {
            markdown.push_str(&format!("   **Description:** {}\n", result.description));
        }
        markdown.push('\n');
    }

    markdown
}

fn parse_duckduckgo_results(html: &str) -> Result<Vec<bindings::SearchResult>, String> {
    let mut results = Vec::new();

    for r#match in RESULT_REGEX.find_iter(html) {
        let html = r#match.as_str();

        if let Some(caps) = LINK_REGEX.captures(html) {
            let url = caps.get(1).unwrap().as_str();
            let title_html = caps.get(2).unwrap().as_str();

            let title = strip_html_tags(title_html);
            let description = extract_description(html);
            let clean_url = clean_duckduckgo_url(url);

            if !title.is_empty() && !clean_url.is_empty() && title.len() > 3 {
                results.push(bindings::SearchResult {
                    title,
                    url: clean_url,
                    description,
                });
            }
        }
    }

    if results.is_empty() {
        results = parse_alternative_results(html)?;
    }

    Ok(results)
}

fn parse_alternative_results(html: &str) -> Result<Vec<bindings::SearchResult>, String> {
    let mut results = Vec::new();

    for caps in ALT_LINK_REGEX.captures_iter(html) {
        let url = caps.get(1).unwrap().as_str();
        let title = strip_html_tags(caps.get(2).unwrap().as_str());
        let clean_url = clean_duckduckgo_url(url);

        if is_valid_result(&title) {
            results.push(bindings::SearchResult {
                title: title.trim().to_string(),
                url: clean_url,
                description: "No description available".to_string(),
            });
        }
    }

    Ok(results)
}

fn extract_description(html: &str) -> String {
    if let Some(caps) = SNIPPET_REGEX.captures(html) {
        strip_html_tags(caps.get(1).unwrap().as_str())
    } else {
        extract_fallback_description(html)
    }
}

fn is_valid_result(title: &str) -> bool {
    !title.is_empty()
        && title.len() > 3
        && !title.contains("All Regions")
        && !title.contains("Any Time")
        && !title.contains("Feedback")
}

fn strip_html_tags(html: &str) -> String {
    HTML_TAG_REGEX
        .replace_all(html, "")
        .replace("&#39;", "'")
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_fallback_description(html: &str) -> String {
    let mut text_parts = Vec::new();
    for caps in TEXT_REGEX.captures_iter(html) {
        if let Some(text) = caps.get(1) {
            let text = text.as_str().trim();
            if !text.is_empty() && text.len() > 10 {
                text_parts.push(text);
            }
        }
    }
    if !text_parts.is_empty() {
        text_parts.join(" ").chars().take(200).collect()
    } else {
        "No description available".to_string()
    }
}

fn clean_duckduckgo_url(url: &str) -> String {
    if url.starts_with("/l/")
        && let Some(caps) = UDDG_REGEX.captures(url)
    {
        return decode(caps.get(1).unwrap().as_str())
            .unwrap_or_default()
            .to_string();
    }

    if url.starts_with("http") {
        url.to_string()
    } else if url.starts_with("/") {
        format!("https://duckduckgo.com{}", url)
    } else {
        format!("https://duckduckgo.com/{}", url)
    }
}

bindings::export!(Component with_types_in bindings);
