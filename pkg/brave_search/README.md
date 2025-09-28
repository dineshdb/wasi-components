# Brave Search Plugin

A WASI component plugin that provides web search functionality using the Brave
Search API.

## Features

- Web search using Brave Search API
- Configurable search parameters (count, country, language, safe search)
- Optional text content from web pages
- Structured search results with titles, URLs, and descriptions

## Usage

### Prerequisites

1. Get a Brave Search API key from
   [Brave Search API](https://api.search.brave.com/)
2. Set the API key as an environment variable:
   ```bash
   export BRAVE_SEARCH_API_KEY=your_api_key_here
   ```

### API

The plugin provides a single function `search` that takes search parameters and
returns search results.

#### Search Parameters

```rust
SearchParams {
    query: String,           // The search query string
    count: u32,              // Number of results (default: 10, max: 20)
    country: String,         // Country code (e.g., "US", "GB")
    language: String,        // Language code (e.g., "en", "es")
    safe_search: String,      // Safe search setting ("off", "moderate", "strict")
    text: bool,              // Whether to include text from web pages
}
```

#### Search Response

```rust
SearchResponse {
    results: Vec<SearchResult>,  // List of search results
    total_results: u32,         // Total number of results available
    query: String,              // The query that was executed
}
```

#### Individual Search Result

```rust
SearchResult {
    title: String,              // Title of the search result
    url: String,                // URL of the search result
    description: String,        // Snippet/description of the result
    text: Option<String>,       // Text content from the page (if requested)
}
```

### Example Usage

```rust
use brave_search::bindings::{SearchParams, SearchResult, SearchResponse};

// Create search parameters
let params = SearchParams {
    query: "rust programming language".to_string(),
    count: 5,
    country: "US".to_string(),
    language: "en".to_string(),
    safe_search: "moderate".to_string(),
    text: false,
};

// Perform search
match brave_search::search(params) {
    Ok(response) => {
        println!("Found {} results for '{}'", response.total_results, response.query);
        for (i, result) in response.results.iter().enumerate() {
            println!("{}. {}", i + 1, result.title);
            println!("   URL: {}", result.url);
            println!("   Description: {}", result.description);
            println!();
        }
    }
    Err(e) => {
        eprintln!("Search failed: {}", e);
    }
}
```

## Building

```bash
# Generate bindings
cargo component bindings

# Build the plugin
cargo build -p brave-search

# Build entire workspace
cargo build --workspace
```

## Dependencies

- `spin-sdk` - For HTTP requests
- `serde` - For JSON serialization/deserialization
- `urlencoding` - For URL encoding of search queries
- `wit-bindgen-rt` - For WIT bindings

## License

MIT License
