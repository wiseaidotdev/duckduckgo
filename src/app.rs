// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CLI Application Entry Point
//!
//! This module contains the top-level async entry function for the `ddg` binary
//! and the Python / Node.js `run_cli` bindings. Extracting the logic here allows
//! the CLI to be driven both from the standalone binary and from language
//! binding `run_cli` helpers without duplicating code.
//!
//! When invoked with no `--query` argument, the function launches the full-screen
//! TUI built with `ratatui` (requires the `tui` Cargo feature which is implied
//! by `rust-binary`).
//!
//! # See Also
//!
//! - [`crate::cli`] - argument definitions parsed by this function.
//! - [`crate::browser`] - HTTP client that executes the actual searches.
//! - [`crate::tui`] - interactive TUI launched when no query is supplied.

use anyhow::Result;
use clap::Parser;

use crate::browser::Browser;
use crate::cli::{Backend, Cli};
use crate::colors::{AnsiColor, AnsiStyle};
use crate::response::ResultFormat;
use crate::user_agents::get;
use urlencoding::encode;

/// Drives the `ddg` CLI from an arbitrary argument list.
///
/// When `--query` / `-q` is omitted, the function checks for the `tui` feature
/// and launches the fullscreen interactive TUI. When a query is present, the
/// corresponding DuckDuckGo backend is called and results are printed to stdout.
///
/// # Arguments
///
/// * `args` - The full argument list in the same format as `std::env::args()`.
///   The first element is conventionally the program name and is consumed by
///   `clap`.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] when a required argument is missing, a proxy
/// URL is invalid, or a network request fails.
pub async fn run_cli_entry(args: Vec<String>) -> Result<()> {
    let args = Cli::parse_from(args);
    let error_style = AnsiStyle {
        bold: true,
        color: Some(AnsiColor::Red),
    };

    if args.query.is_none() {
        #[cfg(feature = "tui")]
        return crate::tui::run_tui().await;

        #[cfg(not(feature = "tui"))]
        {
            eprintln!(
                "{}Error: --query (-q) is required when the TUI feature is disabled.{}",
                error_style.escape_code(),
                AnsiStyle::reset_code()
            );
            std::process::exit(1);
        }
    }

    let query = args.query.unwrap();

    let mut builder = Browser::builder();

    if !args.user_agent.is_empty() {
        match get(&args.user_agent) {
            Some(agent) => builder = builder.user_agent(agent),
            None => {
                eprintln!(
                    "{}Error: Invalid user agent selected!{}",
                    error_style.escape_code(),
                    AnsiStyle::reset_code()
                );
                std::process::exit(1);
            }
        }
    }

    if args.cookie {
        builder = builder.cookie_store(true);
    }

    if !args.proxy.is_empty() {
        builder = builder.proxy(&args.proxy);
    }

    let browser = builder.build()?;
    let usr_agent: &'static str = get(&args.user_agent).unwrap_or("");

    let result_format = if args.format {
        ResultFormat::Detailed
    } else {
        ResultFormat::List
    };

    let limit = Some(args.limit);

    match args.backend {
        Backend::Auto => {
            if !args.operators.is_empty() {
                browser
                    .search_operators(
                        &encode(&query),
                        &encode(&args.operators),
                        args.safe,
                        result_format,
                        limit,
                        None,
                    )
                    .await?;
            } else {
                browser
                    .search(&encode(&query), args.safe, result_format, limit, None)
                    .await?;
            }
        }
        Backend::Lite => {
            let results = browser
                .lite_search(&query, "wt-wt", limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.snippet);
            }
        }
        Backend::Images => {
            let results = browser
                .images(&query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.image);
            }
        }
        Backend::News => {
            let results = browser
                .news(&query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.date, r.title, r.url);
            }
        }
    }

    Ok(())
}
