# DuckDuckGo Python Documentation 🐍

The **`ddg-rs`** package offers blazing-fast, synchronous bindings for the Rust `duckduckgo` library. Unlike many Python network clients, you do not need to deal with `asyncio` or asynchronous loops, the native extension operates a robust `tokio` runtime underneath the covers to ensure optimum performance without the boilerplate.

## 📦 Installation

Install via pip (built with [maturin](https://github.com/PyO3/maturin)):

```sh
# Setup environment
python3 -m venv .venv
source .venv/bin/activate

# Install package
pip install ddg-rs
```

Or build locally:

```sh
pip install maturin
maturin develop --features python
```

## 🛠 Usage Overview

Enter the python interpreter:

```sh
python3
```

### Simple Lite Search

DuckDuckGo Lite is a minimal interface that returns structured results without Javascript execution overheads.

```python
from ddg import Browser

# Create a Browser instance
# Optionally takes `user_agent`, `cookie_store`, and `proxy` arguments.
browser = Browser()

# Execute a lite search globally ("wt-wt")
results = browser.lite_search(query="python programming", region="wt-wt", limit=3)

for result in results:
    print(f"[{result.title}]({result.url})")
    print(f"Summary: {result.snippet}\n")
```

### Image Search

```python
from ddg import Browser

browser = Browser()
images = browser.images(query="ferris the crab", limit=5, safesearch=True)

for img in images:
    print(f"{img.title}")
    print(f"Image: {img.image}")
    print(f"Thumbnail: {img.thumbnail}\n")
```

### News Search

```python
from ddg import Browser

browser = Browser()
news_items = browser.news(query="technology news", limit=5, safesearch=False)

for news in news_items:
    print(f"[{news.date}] {news.title}")
    print(f"{news.url}\n")
```

### Instant Answer API

For retrieving abstract encyclopedia summaries, structured calculations, or specific direct answers from the Instant Answer API without parsing HTML.

```python
from ddg import Browser, SearchParams

browser = Browser()
params = SearchParams().safe_search("off")

# Instant Answers API returns a dictionary payload
response = browser.instant_answer("python language", params=params)

print(response["heading"])
print(response["abstract_text"])
```

## 🔍 Setting Up Advanced URL Parameters

`SearchParams` provides a fluent configuration builder for passing additional query logic or theme styling to DuckDuckGo formats. You can chain these methods.

```python
from ddg import Browser, SearchParams

# Configure DuckDuckGo settings
params = (
    SearchParams()
    .region("us-en")
    .safe_search("moderate")
    .theme("dark")
    .source("my_python_app")
)

browser = Browser()
response = browser.instant_answer("encryption", params=params)
```

## 📖 Component API Reference

### `Browser` Class

Initializes the search engine.

- `Browser(user_agent: typing.Optional[str] = None, cookie_store: bool = False, proxy: typing.Optional[str] = None)`

**Methods:**

- `lite_search(query: str, region: str = "wt-wt", limit: typing.Optional[int] = None, user_agent: str = "") -> list[LiteSearchResult]`
- `images(query: str, region: str = "wt-wt", safesearch: bool = True, limit: typing.Optional[int] = None, user_agent: str = "") -> list[ImageResult]`
- `news(query: str, region: str = "wt-wt", safesearch: bool = True, limit: typing.Optional[int] = None, user_agent: str = "") -> list[NewsResult]`
- `instant_answer(query: str, params: typing.Optional[SearchParams] = None) -> dict`

### Result Types

- **`LiteSearchResult`**: `title` (str), `url` (str), `snippet` (str).
- **`ImageResult`**: `title` (str), `image` (str), `thumbnail` (str), `url` (str), `height` (int), `width` (int), `source` (str).
- **`NewsResult`**: `date` (str), `title` (str), `body` (str), `url` (str), `image` (str | None), `source` (str).

### `SearchParams` Class

Configuration builder for standard search payloads. Returns a new instance with each method.

- `region(code: str) -> SearchParams` (e.g. `us-en`, `fr-fr`. See DDG param docs)
- `safe_search(level: str) -> SearchParams` (`on`, `moderate`, `off`)
- `theme(name: str) -> SearchParams` (`default`, `contrast`, `retro`, `dark`, `terminal`)
- `source(src: str) -> SearchParams`
- `header_color(color: str) -> SearchParams`
- `url_color(color: str) -> SearchParams`
- `background_color(color: str) -> SearchParams`
- `text_color(color: str) -> SearchParams`
- `link_color(color: str) -> SearchParams`
- `visited_link_color(color: str) -> SearchParams`
- `to_query_pairs() -> list[tuple[str, str]]`
