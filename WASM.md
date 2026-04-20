# WebAssembly (WASM) Support 🌐

The `duckduckgo` crate natively supports compiling to `wasm32-unknown-unknown`, making it an excellent drop-in solution for client-side Rust frontend applications. This allows you to leverage the full DuckDuckGo search API directly from the browser!

## Framework Compatibility

Because the internal HTTP layer automatically swaps `reqwest` out for its browser-fetch implementation under the hood, `duckduckgo` works out-of-the-box with all major Rust frontend frameworks:

- **[Yew](https://yew.rs/)**
- **[Dioxus](https://dioxuslabs.com/)**
- **[Leptos](https://leptos.dev/)**
- **[Sycamore](https://sycamore.dev/)**

## 📦 Usage

To use `duckduckgo` in your WASM project, simply add it to your `Cargo.toml`. By default, the required WASM dependencies (`js-sys`, `wasm-bindgen`, and the WASM `reqwest` HTTP backend) will be correctly resolved.

```toml
[dependencies]
duckduckgo = "0.2.3"
```

## Example: Building a Chat Interface

You can utilise `duckduckgo` as a powerful search heuristic backend for agents running on the web.
A reference implementation demonstrating complex WASM asynchronous search capabilities inside a Yew application can be found in the **[llm/examples/chat](https://github.com/wiseaidotdev/lmm/tree/main/examples/chat)** repository.

### Minimal Example (Yew)

```rust
use yew::prelude::*;
use yew::platform::spawn_local;
use duckduckgo::browser::Browser;
use duckduckgo::response::ResultFormat;

#[function_component(App)]
pub fn app() -> Html {
    let results = use_state(|| vec![]);

    let onclick = {
        let results = results.clone();
        Callback::from(move |_| {
            let results = results.clone();
            spawn_local(async move {
                let browser = Browser::new();
                if let Ok(data) = browser.lite_search("Rust WebAssembly", "wt-wt", Some(3), "").await {
                    results.set(data.into_iter().map(|r| r.title).collect());
                }
            });
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Search DuckDuckGo" }</button>
            <ul>
                { for results.iter().map(|title| html! { <li>{ title }</li> }) }
            </ul>
        </div>
    }
}
```
