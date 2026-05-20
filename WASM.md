# WebAssembly (WASM) Support 🌐

The `duckduckgo` crate natively supports compiling to `wasm32-unknown-unknown`, making it an excellent drop-in solution for client-side Rust frontend applications. This allows you to leverage the full DuckDuckGo search API directly from the browser!

## Framework Compatibility

Because the internal HTTP layer automatically swaps `reqwest` out for its browser-fetch implementation under the hood, `duckduckgo` works out-of-the-box with all major Rust frontend frameworks:

- **[Yew](https://yew.rs/)**
- **[Dioxus](https://dioxuslabs.com/)**
- **[Leptos](https://leptos.dev/)**
- **[Sycamore](https://sycamore.dev/)**

## 4-Tier Search Strategy (Automatic in WASM)

`api.duckduckgo.com` does not reliably include `Access-Control-Allow-Origin` headers in browser contexts. The first request often succeeds, but subsequent requests are often blocked by the browser's CORS enforcement.

**The `duckduckgo` crate addresses this in WASM builds using a robust 4-tier fallback strategy:**

1.  **Direct DDG Call**: Attempts a direct fetch. This often succeeds on the first request if the browser has cached preflight headers.
1.  **Self-hosted CORS Proxy**: If configured via the `DDG_CORS_PROXY_URL` environment variable at build time, the request is routed through a proxy you control. This is the most reliable way to get DDG results in production.
1.  **Public CORS Proxies**: If no self-hosted proxy is provided, the library automatically attempts a sequence of reliable public proxies (like `corsproxy.io`) to fetch your results.
1.  **Wikipedia API Fallback**: A guaranteed CORS-safe fallback using Wikipedia's search and extract APIs (with `origin=*`).

### Configuring a Production Proxy

To use a self-hosted proxy, set the environment variable at **build time**:

```sh
DDG_CORS_PROXY_URL=https://your-proxy.workers.dev trunk build
```

Or add it to your project's `.cargo/config.toml`:

```toml
[env]
DDG_CORS_PROXY_URL = "https://your-proxy.workers.dev"
```

> **Note:** `lite.duckduckgo.com` (used by [`Browser::lite_search`]) explicitly returns `403 Forbidden` to CORS preflight requests. Avoid calling it from WASM; use [`Browser::get_api_response`] instead, which is CORS-safe via the proxy.

## 📦 Usage

To use `duckduckgo` in your WASM project, simply add it to your `Cargo.toml`. By default, the required WASM dependencies (`js-sys`, `wasm-bindgen`, and the WASM `reqwest` HTTP backend) will be correctly resolved.

```toml
[dependencies]
duckduckgo = "0.3.3"
```

### Minimal Example (Yew)

```rust
use yew::prelude::*;
use yew::platform::spawn_local;
use duckduckgo::browser::Browser;

#[function_component(App)]
pub fn app() -> Html {
    let answer = use_state(String::new);

    let onclick = {
        let answer = answer.clone();
        Callback::from(move |_| {
            let answer = answer.clone();
            spawn_local(async move {
                let browser = Browser::new();
                if let Ok(resp) = browser.get_api_response("?q=Rust+programming", None).await {
                    if let Some(text) = resp.abstract_text {
                        answer.set(text);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Ask DuckDuckGo" }</button>
            <p>{ (*answer).clone() }</p>
        </div>
    }
}
```

## Example: Building a Chat Interface

You can utilise `duckduckgo` as a powerful search heuristic backend for agents running on the web.
A reference implementation demonstrating complex WASM asynchronous search capabilities inside a Yew application can be found in the **[llm/examples/chat](https://github.com/wiseaidotdev/lmm/tree/main/examples/chat)** repository.
