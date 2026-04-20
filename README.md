<div align="center">

# 🦆 Duckduckgo

[![DDG](https://raw.githubusercontent.com/wiseaidotdev/duckduckgo/refs/heads/main/assets/logo.png)](https://github.com/wiseaidotdev/duckduckgo)

[![Crates.io](https://img.shields.io/crates/v/duckduckgo.svg)](https://crates.io/crates/duckduckgo)
[![Docs.rs](https://docs.rs/duckduckgo/badge.svg)](https://docs.rs/duckduckgo)
[![npm](https://img.shields.io/npm/v/ddg-rs.svg)](https://www.npmjs.com/package/ddg-rs)
[![PyPI](https://img.shields.io/pypi/v/ddg-rs.svg)](https://pypi.org/project/ddg-rs)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/wiseaidotdev/duckduckgo/blob/main/LICENSE)

> `duckduckgo` is a multi-language toolkit for searching DuckDuckGo from code or the command line. The core is written entirely in Rust and compiled to a native extension, so Python and Node.js callers enjoy the same performance and correctness guarantees as the Rust library, with no runtime dependencies beyond the native extension itself 🗿.

</div>

## 🤔 What does this crate provide?

The library exposes 4 search backends:

- **Instant Answer**: the official `api.duckduckgo.com` JSON endpoint, which
  returns structured data including abstracts, definitions, direct answers, and
  related topics.
- **Lite**: the minimal `lite.duckduckgo.com` HTML interface, which returns
  plain text results without JavaScript.
- **Images**: paginated image results via `duckduckgo.com/i.js`.
- **News**: paginated news articles via `duckduckgo.com/news.js`.

A comprehensive [`SearchParams`](https://docs.rs/duckduckgo/latest/duckduckgo/params/struct.SearchParams.html)
builder covers every documented DuckDuckGo URL parameter: region, safe-search,
theme, colours, fonts, header behaviour, and more.

## 🦀 Rust

<!-- absolute url for docs.rs cause RUST.md is not included in crate for minimal bundle size -->

The Rust crate is available on [crates.io](https://crates.io/crates/duckduckgo).
For a complete API reference, installation guide, and worked examples, visit the
**[Rust Usage Guide](https://github.com/wiseaidotdev/duckduckgo/blob/main/RUST.md)**.

The crate ships the following Cargo features:

| Feature       | Description                                          |
| ------------- | ---------------------------------------------------- |
| `rust-binary` | Enables the standalone `ddg` terminal CLI executable |
| `python`      | Python extension module via `pyo3`                   |
| `node`        | Node.js native add-on via `napi-derive`              |

## 🌐 WebAssembly (WASM)

`duckduckgo` natively supports the `wasm32-unknown-unknown` target! Because it uses `reqwest` under the hood, it seamlessly switches to the `fetch` API when deployed in the browser.

This makes it perfect for client-side search inside Rust frontend frameworks like **Yew**, **Dioxus**, and **Leptos**. We actively use this technique in the [`llm/examples/chat`](https://github.com/wiseaidotdev/lmm/tree/main/examples/chat) application.

<!-- absolute url for docs.rs cause WASM.md is not included in crate for minimal bundle size -->

For CORS considerations, code examples, and usage details, read the
**[WASM usage guide](https://github.com/wiseaidotdev/duckduckgo/blob/main/WASM.md)**.

## 🐍 Python

The Python bindings are published to PyPI as **`ddg-rs`** and can be installed
with `pip install ddg-rs`. The package is built with
[maturin](https://www.maturin.rs/) and ships pre-compiled wheels for the major
CPython versions.

The Python module exposes the `Browser`, `SearchParams`, `LiteSearchResult`,
`ImageResult`, and `NewsResult` types. All network methods are synchronous —
they drive an embedded Tokio runtime so callers do not need an event loop.

<!-- absolute url for docs.rs cause PYTHON.md is not included in crate for minimal bundle size -->

For installation instructions, configuration options, and full method
signatures, read the **[Python usage guide](https://github.com/wiseaidotdev/duckduckgo/blob/main/PYTHON.md)**.

## 🟩 Node.js

The Node.js bindings are published to npm as **`ddg-rs`** and can be installed
with `npm install ddg-rs`. The package is built with
[napi-rs](https://napi.rs/) and ships a pre-compiled `.node` add-on.

The module exposes the `Browser` class, `SearchParams` builder, and plain
object result types (`LiteSearchResult`, `ImageResult`, `NewsResult`,
`InstantAnswerResponse`). Like the Python API, all network calls are
synchronous within the binding.

<!-- absolute url for docs.rs cause NODE.md is not included in crate for minimal bundle size -->

For installation instructions, type definitions, and examples, read the
**[Node.js usage guide](https://github.com/wiseaidotdev/duckduckgo/blob/main/NODE.md)**.

## 💻 Command-line interface

The `ddg` binary supports full-text, image, news, and Instant Answer searches
directly from the terminal. It accepts a rich set of options including
user-agent selection, cookie storage, HTTP proxy support, safe-search, and a
choice of result backends.

<!-- absolute url for docs.rs cause CLI.md is not included in crate for minimal bundle size -->

For the full option reference and usage examples, see the
**[CLI documentation](https://github.com/wiseaidotdev/duckduckgo/blob/main/CLI.md)** or run `ddg --help` after installing with
`cargo install duckduckgo --features rust-binary`.

## 🔒 Privacy

DuckDuckGo does not track users or personalise results. This library makes
direct HTTPS requests to the same public endpoints that the browser does. No
API key, account, or authentication token is required.

For more information about DuckDuckGo's privacy practices, visit
[duckduckgo.com/privacy](https://duckduckgo.com/privacy).

## 📚 Further reading

- [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
- [DuckDuckGo URL Parameters reference](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
- [DuckDuckGo Search Syntax](https://help.duckduckgo.com/duckduckgo-help-pages/results/syntax/)
- [DuckDuckGo Help Pages](https://duckduckgo.com/duckduckgo-help-pages/)
- [Instant Answer API playground](https://api.duckduckgo.com/?q=rust&format=json&pretty=1)

## 📄 License

Licensed under the [MIT License](LICENSE).

## ⭐ Star us

If you use or enjoy this toolkit, please leave us a star on [GitHub](https://github.com/wiseaidotdev/duckduckgo)! It helps others discover the project and keeps the momentum going and the coffee flowing ☕.

[![Star History Chart](https://api.star-history.com/svg?repos=wiseaidotdev/duckduckgo&type=Date)](https://star-history.com/#wiseaidotdev/duckduckgo&Date)
