# DuckDuckGo Rust Documentation 🦀

The `duckduckgo` library is a fast, privacy-respecting pure-Rust toolkit for querying DuckDuckGo. It exposes four search backends, a comprehensive parameter builder, and is designed for seamless async usage within the `tokio` ecosystem 🎌.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
duckduckgo = "0.2.3"
```

## 📚 Library Usage

### Simple browser (zero-configuration)

```rust
use duckduckgo::browser::Browser;
use duckduckgo::response::ResultFormat;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::new();
    browser.search("rust lang", true, ResultFormat::List, Some(5), None).await?;
    Ok(())
}
```

### Configuring the browser with `BrowserBuilder`

```rust
use duckduckgo::browser::Browser;
use duckduckgo::response::ResultFormat;
use duckduckgo::user_agents::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_agent = get("firefox").unwrap();

    let browser = Browser::builder()
        .user_agent(user_agent)
        .cookie_store(true)
        // .proxy("socks5://127.0.0.1:9050")  // optional
        .build()?;

    browser.search("rust lang", true, ResultFormat::Detailed, Some(5), None).await?;
    Ok(())
}
```

### Image search

Retrieves paginated image results via `duckduckgo.com/i.js`.

```rust
use duckduckgo::browser::Browser;
use duckduckgo::user_agents::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::new();
    let images = browser.images("ferris crab", "us-en", true, Some(3), get("firefox").unwrap()).await?;
    for img in images {
        println!("{}: {}", img.title, img.image);
    }
    Ok(())
}
```

### News search

Retrieves paginated news articles via `duckduckgo.com/news.js`.

```rust
use duckduckgo::browser::Browser;
use duckduckgo::user_agents::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::new();
    let news = browser.news("Rust programming", "us-en", true, Some(5), get("firefox").unwrap()).await?;
    for article in news {
        println!("[{}] {} - {}", article.date, article.title, article.url);
    }
    Ok(())
}
```

### Lite search

Retrieves plain text HTML results from the `lite.duckduckgo.com` proxy endpoint without requiring JavaScript.

```rust
use duckduckgo::browser::Browser;
use duckduckgo::user_agents::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::new();
    let results = browser.lite_search("systems programming", "wt-wt", Some(3), get("firefox").unwrap()).await?;
    for r in results {
        println!("{}\n{}", r.title, r.url);
    }
    Ok(())
}
```

## 🔧 Search Parameters

The `params` module provides strongly-typed enums for every documented DuckDuckGo URL parameter. Pass a `SearchParams` instance to `search()`, `advanced_search()`, or `search_operators()`.

```rust
use duckduckgo::browser::Browser;
use duckduckgo::response::ResultFormat;
use duckduckgo::params::{SearchParams, Region, SafeSearch, Theme, Size, Font, Toggle};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let params = SearchParams::new()
        .region(Region::UsEn)
        .safe_search(SafeSearch::Moderate)
        .theme(Theme::Dark)
        .size(Size::Large)
        .link_font(Font::Verdana)
        .full_urls(Toggle::On)
        .source("my_app");

    let browser = Browser::new();
    browser.search("rust lang", false, ResultFormat::List, Some(5), Some(&params)).await?;
    Ok(())
}
```

### Available Parameter Groups

| Group           | Key Methods                                                                                                                                                         |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Result**      | `region(Region)`, `safe_search(SafeSearch)`, `open_instant_answers`, `auto_load_images`, `auto_load_results`, `new_window`, `favicons`, `full_urls`, `auto_suggest` |
| **Privacy**     | `redirect`, `https`, `address_bar(AddressBar)`, `video_playback(VideoPlayback)`                                                                                     |
| **Colour**      | `header_color`, `url_color`, `background_color`, `text_color`, `link_color`, `visited_link_color`                                                                   |
| **Look & Feel** | `theme(Theme)`, `size(Size)`, `width(Width)`, `placement(Placement)`, `link_font(Font)`, `text_font(Font)`, `underline`                                             |
| **Interface**   | `header_behavior(HeaderBehavior)`, `advertisements`, `page_numbers(PageNumbers)`, `units_measure(UnitsMeasure)`                                                     |
| **Source**      | `source(impl Into<String>)`                                                                                                                                         |

## 🔍 Instant Answer API

You can use `get_api_response` to retrieve full structured answers mapping to the JSON Instant Answers API.

```rust
use duckduckgo::browser::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::new();
    let response = browser.get_api_response("?q=rust+lang", None).await?;
    println!("Heading: {:?}", response.heading);
    println!("Abstract: {:?}", response.abstract_text);
    Ok(())
}
```

## Supported Core Types

| Struct             | Purpose                                                          |
| ------------------ | ---------------------------------------------------------------- |
| `Browser`          | Main client to dispatch requests. Contains embedded HTTP client. |
| `BrowserBuilder`   | Client configuration for proxy, headers, and cookies.            |
| `SearchParams`     | Comprehensive DuckDuckGo URL search parameter builder.           |
| `Response`         | Type mapping for Instant Answer API schemas.                     |
| `LiteSearchResult` | Datastructure for returned minimal HTML lite search items.       |
| `ImageResult`      | Datastructure for returned `/i.js` image entries.                |
| `NewsResult`       | Datastructure for returned `/news.js` article entries.           |
