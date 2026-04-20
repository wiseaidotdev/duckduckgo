# DuckDuckGo Node.js Documentation 🟩

The **`ddg-rs`** module offers extremely fast, native synchronous bindings for the Rust `duckduckgo` library via N-API / napi-rs. Because the library maintains its own highly optimized, embedded native executor (Tokio), N-API functions run blisteringly fast while providing an intuitive, blocking-style Javascript API that saves you from deep async closures and Promises for one-off search calls.

## 📦 Installation

Install via npm (built with [napi-rs](https://napi.rs)):

```sh
npm install ddg-rs
```

Or build locally:

```sh
npm install -g @napi-rs/cli
napi build --platform --release --features node
```

## 🛠 Usage Overview

Enter the node interpreter:

```sh
node
```

### Lite Search

Provides structured HTML response extraction using DuckDuckGo's ultra-minimal Javascript-free fallback.

```javascript
// If installed via npm: const { Browser } = require("ddg-rs");
// For local development:
const { Browser } = require(".");

// Browser optionally accepts (userAgent, cookieStore, proxy)
const browser = new Browser();

// Run a synchronized search
const results = browser.liteSearch(
  "javascript frameworks",
  "wt-wt", // region
  5, // limit
  "", // default user agent
);

for (const res of results) {
  console.log(`[${res.title}](${res.url})`);
  console.log(`${res.snippet}\n`);
}
```

### Image Search

```javascript
// If installed via npm: const { Browser } = require("ddg-rs");
// For local development:
const { Browser } = require(".");

const browser = new Browser();
const images = browser.images("ferris the crab", "us-en", true, 3);

for (const img of images) {
  console.log(`${img.title}`);
  console.log(`Full: ${img.image} | Thumb: ${img.thumbnail}`);
}
```

### News Search

```javascript
// If installed via npm: const { Browser } = require("ddg-rs");
// For local development:
const { Browser } = require(".");

const browser = new Browser();
const articles = browser.news("tech startups", "us-en", true, 3);

for (const news of articles) {
  console.log(`[${news.date}] ${news.title}`);
  console.log(`${news.url}\n`);
}
```

### Instant Answer API

Fetches encyclopedic responses, math calculations, specific abstract entity answers directly.

```javascript
// If installed via npm: const { Browser, SearchParams } = require("ddg-rs");
// For local development:
const { Browser, SearchParams } = require(".");

const browser = new Browser();
const params = new SearchParams().safeSearch("off");

const response = browser.instantAnswer("rust programming language", params);

console.log(response.heading);
console.log(response.abstractText);
```

## 🔍 Configuring Search Parameters

Filter results or supply API requirements via the chainable `SearchParams` entity map.

```javascript
// If installed via npm: const { Browser, SearchParams } = require("ddg-rs");
// For local development:
const { Browser, SearchParams } = require(".");

const params = new SearchParams()
  .region("us-en")
  .safeSearch("moderate")
  .theme("dark")
  .source("my_node_cli");

const browser = new Browser();
const response = browser.instantAnswer("nodejs event loop", params);
```

## 📖 Component API Reference

### `Browser` Entity

- `new Browser(userAgent?: string, cookieStore?: boolean, proxy?: string)`

**Methods:**

- `liteSearch(query: string, region?: string, limit?: number, userAgent?: string): LiteSearchResult[]`
- `images(query: string, region?: string, safesearch?: boolean, limit?: number, userAgent?: string): ImageResult[]`
- `news(query: string, region?: string, safesearch?: boolean, limit?: number, userAgent?: string): NewsResult[]`
- `instantAnswer(query: string, params?: SearchParams): InstantAnswerResponse`

### Value Objects

- **`LiteSearchResult`**: `{ title: string, url: string, snippet: string }`
- **`ImageResult`**: `{ title: string, image: string, thumbnail: string, url: string, height: number, width: number, source: string }`
- **`NewsResult`**: `{ date: string, title: string, body: string, url: string, image: string | undefined, source: string }`

### `InstantAnswerResponse` Type

Complete schema breakdown matching the `api.duckduckgo.com` JSON specification:

- `heading` (string)
- `abstractText` (string)
- `abstractSource` (string)
- `abstractUrl` (string)
- `answer` (string)
- `answerType` (string)
- `definition` (string)
- `definitionSource` (string)
- `definitionUrl` (string)
- `entity` (string)
- `image` (string)
- `redirect` (string)
- `responseType` (string)
- `relatedTopics` (Array of `<{ text?: string, firstUrl?: string, url?: string, result?: string }>`)

### `SearchParams` Class

Helper for generating standard browser payload metadata.

- `region(code: string): this`
- `safeSearch(level: string): this` -> (`on`, `moderate`, `off`)
- `theme(name: string): this`
- `source(src: string): this`
- `headerColor(color: string): this`
- `urlColor(color: string): this`
- `backgroundColor(color: string): this`
- `textColor(color: string): this`
- `linkColor(color: string): this`
- `visitedLinkColor(color: string): this`
- `toQueryPairs(): string[][]`
