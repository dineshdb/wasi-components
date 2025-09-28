use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraveResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub mixed: MixedResults,
    pub query: QueryInfo,
    pub videos: Option<VideoResults>,
    pub web: WebResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedResults {
    #[serde(rename = "type")]
    pub results_type: String,
    pub main: Vec<MainItem>,
    pub side: Vec<serde_json::Value>, // Assuming side can be various or empty
    pub top: Vec<serde_json::Value>,  // Assuming top can be various or empty
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainItem {
    pub all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInfo {
    pub bad_results: bool,
    pub city: String,
    pub country: String,
    pub header_country: String,
    pub is_navigational: bool,
    pub is_news_breaking: bool,
    pub more_results_available: bool,
    pub original: String,
    pub postal_code: String,
    pub should_fallback: bool,
    pub show_strict_warning: bool,
    pub spellcheck_off: bool,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResults {
    #[serde(rename = "type")]
    pub results_type: String,
    pub mutated_by_goggles: bool,
    pub results: Vec<VideoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub age: String,
    pub description: String,
    pub fetched_content_timestamp: u64,
    pub meta_url: MetaUrl,
    pub page_age: String,
    pub thumbnail: Thumbnail,
    pub title: String,
    pub url: String,
    pub video: VideoDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetails {
    pub creator: String,
    pub duration: String,
    pub publisher: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebResults {
    #[serde(rename = "type")]
    pub results_type: String,
    pub family_friendly: bool,
    pub results: Vec<WebItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebItem {
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Vec<ClusterItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    pub description: String,
    pub family_friendly: bool,
    pub is_live: bool,
    pub is_source_both: bool,
    pub is_source_local: bool,
    pub language: String,
    pub meta_url: MetaUrl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterItem {
    pub description: String,
    pub family_friendly: bool,
    pub is_source_both: bool,
    pub is_source_local: bool,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaUrl {
    pub favicon: String,
    pub hostname: String,
    pub netloc: String,
    pub path: String,
    pub scheme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub img: String,
    pub long_name: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<bool>,
    pub original: String,
    pub src: String,
}
