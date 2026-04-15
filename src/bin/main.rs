use anyhow::Result;
#[cfg(feature = "cli")]
use {
    clap::Parser,
    duckduckgo::browser::Browser,
    duckduckgo::cli::{Backend, Cli},
    duckduckgo::colors::{AnsiColor, AnsiStyle},
    duckduckgo::response::ResultFormat,
    duckduckgo::user_agents::get,
    urlencoding::encode,
};

/// The main entry point of the DuckDuckGo search CLI application.
///
/// Parses command-line arguments using `clap`, builds a [`Browser`] instance via
/// [`Browser::builder()`], and performs a DuckDuckGo search using the specified query
/// and optional operators.
///
/// # Arguments
/// * `--user-agent` - Specify a custom User-Agent for HTTP requests. Default: `"firefox"`.
/// * `--cookie`     - Enable cookie storage for HTTP requests.
/// * `--proxy`      - Specify an HTTP proxy URL (e.g. `"socks5://192.168.1.1:9000"`).
/// * `--format`     - Enable detailed result format (default is list format).
/// * `--limit`      - Limit the number of search results displayed.
/// * `--query`      - The search query.
/// * `--operators`  - Optional search operators (e.g. `"site:github.com"`).
/// * `--safe`       - Enable safe search mode.
/// * `--backend`    - Backend to use: `Auto`, `Lite`, `Images`, or `News`.
///
/// # Examples
/// ```
/// // Basic search
/// ddg --query "Rust"
///
/// // Operator search with result limit
/// ddg --query "Rust" --operators "site:github.com" --limit 5
/// ```
///
/// # Errors
/// The function exits with a non-zero status code when required arguments are missing or
/// the browser could not be constructed.
#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let style = AnsiStyle {
        bold: true,
        color: Some(AnsiColor::Red),
    };

    let mut builder = Browser::builder();

    if !args.user_agent.is_empty() {
        if let Some(agent) = get(&args.user_agent[..]) {
            builder = builder.user_agent(agent);
        } else {
            eprintln!(
                "{}Error: Invalid user agent selected!{}",
                style.escape_code(),
                AnsiStyle::reset_code()
            );
            std::process::exit(1);
        }
    }

    if args.cookie {
        builder = builder.cookie_store(true);
    }

    if !args.proxy.is_empty() {
        builder = builder.proxy(&args.proxy);
    }

    let browser = builder.build()?;

    let usr_agent: &'static str = get(&args.user_agent[..]).unwrap_or("");

    let result_format = if args.format {
        ResultFormat::Detailed
    } else {
        ResultFormat::List
    };

    let limit = Some(args.limit);

    if args.query.is_empty() {
        eprintln!(
            "{}Error: Query is required!{}",
            style.escape_code(),
            AnsiStyle::reset_code()
        );
        std::process::exit(1);
    }

    match args.backend {
        Backend::Auto => {
            if !args.operators.is_empty() {
                browser
                    .search_operators(
                        &encode(&args.query),
                        &encode(&args.operators),
                        args.safe,
                        result_format,
                        limit,
                        None,
                    )
                    .await?;
            } else {
                browser
                    .search(&encode(&args.query), args.safe, result_format, limit, None)
                    .await?;
            }
        }
        Backend::Lite => {
            let results = browser
                .lite_search(&args.query, "wt-wt", limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.snippet);
            }
        }
        Backend::Images => {
            let results = browser
                .images(&args.query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.image);
            }
        }
        Backend::News => {
            let results = browser
                .news(&args.query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.date, r.title, r.url);
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() -> Result<()> {
    Ok(())
}
