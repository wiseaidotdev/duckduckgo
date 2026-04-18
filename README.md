# 🦆 DuckDuckGo

[![Crates.io](https://img.shields.io/crates/v/duckduckgo.svg)](https://crates.io/crates/duckduckgo)
[![docs](https://docs.rs/duckduckgo/badge.svg)](https://docs.rs/duckduckgo/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```sh
██████╗ ██████╗  ██████╗
██╔══██╗██╔══██╗██╔════╝
██║  ██║██║  ██║██║  ███╗
██║  ██║██║  ██║██║   ██║
██████╔╝██████╔╝╚██████╔╝
╚═════╝ ╚═════╝  ╚═════╝

Search and advanced search in DuckDuckGo
========================================
```

## 📖 Table of Contents

- [Installation](#-installation)
- [Features](#-features)
- [Library Usage](#-library-usage)
- [Search Parameters](#-search-parameters)
- [Usage](#-usage)
- [Options](#-options)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `duckduckgo` cli, use the following Cargo command:

```bash
cargo install --locked duckduckgo --all-features
```

To use it as a library, add it to your `Cargo.toml`:

```toml
[dependencies]
duckduckgo = "0.2.1"
```

## ✨ Features

- 🔍 **Simple and advanced search**: Perform DuckDuckGo searches using basic queries or advanced search operators.
- ⚙️ **Search operators**: Refine results using DuckDuckGo-compatible filters (e.g. site, filetype, intitle).
- 🛡️ **Safe search toggle**: Enable or disable family-friendly search filtering.
- 🖨️ **Output formatting**: Choose between list or detailed formats for displaying results.
- 🧮 **Result limiting**: Limit the number of results returned (default is 10).
- 🧭 **Backend selection**: Use `--backend` to choose the search backend (`Auto`, `HTML`, or `API`).
- 🌐 **Custom user agent**: Spoof or specify a user agent string for requests.
- 🧩 **Cookie control**: Enable or disable cookie handling for requests.
- 🕵️ **Proxy support**: Route requests through a custom HTTP or SOCKS proxy.
- 🐛 **Verbose mode**: Print debug information for troubleshooting.
- 🎨 **ANSI-colored output**: Enjoy beautiful, readable output right in your terminal.
- 🔧 **Full URL parameter support**: Typed enums for every DuckDuckGo URL setting (region, theme, font, colour, ...).

## 🚗 CLI Usage

Learn how to use the duckduckgo and explore its features with the following examples:

### Perform a basic search:

```bash
ddg --query "rust lang"
```

### Use search operators:

```bash
ddg --query "rust lang" --operators "+tutorial"
```

### Enable safe search:

```bash
ddg --query "rust lang" --safe
```

### Set the output format to detailed:

```bash
ddg --query "rust lang" --format
```

### Limit the number of results to 3:

```bash
ddg --query "rust lang" --limit 3
```

### Set user agent:

```bash
ddg --query "rust lang" --user-agent "chrome"
```

### Set Different Backends:

```bash
# news
ddg --query "rust lang" --backend news

# images
ddg --query "rust lang" --backend images

# lite
ddg --query "rust lang" --backend lite
```

<details>
<summary><code>Available Browsers/Agents</code></summary>

| Browser/Agent          |
| ---------------------- |
| abrowse                |
| acoo browser           |
| america online browser |
| amigavoyager           |
| aol                    |
| arora                  |
| avant browser          |
| beonex                 |
| bonecho                |
| browzar                |
| camino                 |
| charon                 |
| cheshire               |
| chimera                |
| chrome                 |
| chromeplus             |
| classilla              |
| cometbird              |
| comodo_dragon          |
| conkeror               |
| crazy browser          |
| cyberdog               |
| deepnet explorer       |
| deskbrowse             |
| dillo                  |
| dooble                 |
| edge                   |
| element browser        |
| elinks                 |
| enigma browser         |
| enigmafox              |
| epiphany               |
| escape                 |
| firebird               |
| firefox                |
| fireweb navigator      |
| flock                  |
| fluid                  |
| galaxy                 |
| galeon                 |
| granparadiso           |
| greenbrowser           |
| hana                   |
| hotjava                |
| ibm webexplorer        |
| ibrowse                |
| icab                   |
| iceape                 |
| icecat                 |
| iceweasel              |
| inet browser           |
| internet explorer      |
| irider                 |
| iron                   |
| k-meleon               |
| k-ninja                |
| kapiko                 |
| kazehakase             |
| kindle browser         |
| kkman                  |
| kmlite                 |
| konqueror              |
| leechcraft             |
| links                  |
| lobo                   |
| lolifox                |
| lorentz                |
| lunascape              |
| lynx                   |
| madfox                 |
| maxthon                |
| midori                 |
| minefield              |
| mozilla                |
| myibrow                |
| myie2                  |
| namoroka               |
| navscape               |
| ncsa_mosaic            |
| netnewswire            |
| netpositive            |
| netscape               |
| netsurf                |
| omniweb                |
| opera                  |
| orca                   |
| oregano                |
| osb-browser            |
| palemoon               |
| phoenix                |
| pogo                   |
| prism                  |
| qtweb internet browser |
| rekonq                 |
| retawq                 |
| rockmelt               |
| safari                 |
| seamonkey              |
| shiira                 |
| shiretoko              |
| sleipnir               |
| slimbrowser            |
| stainless              |
| sundance               |
| sunrise                |
| surf                   |
| sylera                 |
| tencent traveler       |
| tenfourfox             |
| theworld browser       |
| uzbl                   |
| vimprobable            |
| vonkeror               |
| w3m                    |
| weltweitimnetzbrowser  |
| worldwideweb           |
| wyzo                   |

</details>

### Set cookie for subsequent requests:

```bash
ddg --query "rust lang" --cookie
```

### Set proxy:

```bash
ddg --query "rust lang" --proxy "socks5://192.168.1.1:9000"
```

### 🎨 Options

| Option               | Default Value | Description                                                   |
| -------------------- | ------------- | ------------------------------------------------------------- |
| `--query`, `-q`      | _(required)_  | Set the search query.                                         |
| `--operators`, `-o`  | `""`          | Set search operators (e.g., `+site:rust-lang.org`).           |
| `--safe`, `-s`       | `false`       | Enable safe search (family-friendly results).                 |
| `--format`, `-f`     | `false`       | Output format: `false` = list, `true` = detailed.             |
| `--limit`, `-l`      | `10`          | Limit the number of search results.                           |
| `--user-agent`, `-u` | `firefox`     | Set a custom user agent for HTTP requests.                    |
| `--cookie`, `-c`     | `true`        | Enable cookie handling for the HTTP client.                   |
| `--proxy`, `-p`      | `""`          | Set a proxy for requests (e.g., `socks5://192.168.1.1:9000`). |
| `--backend`, `-b`    | `Auto`        | Choose backend: `Auto`, `Lite`, `Images`, or `News`.          |
| `--verbose`, `-v`    | `false`       | Enable verbose (debug) mode.                                  |

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

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidotdev/duckduckgo).
Your contributions help improve this CLI for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
