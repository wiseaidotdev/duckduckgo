use clap::Parser;
use clap::ValueEnum;
use clap::builder::styling::{AnsiColor, Effects, Styles};

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum Backend {
    Auto,
    Lite,
    Images,
    News,
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "duckduckgo",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{about}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about=r#"
██████╗ ██████╗  ██████╗ 
██╔══██╗██╔══██╗██╔════╝ 
██║  ██║██║  ██║██║  ███╗
██║  ██║██║  ██║██║   ██║
██████╔╝██████╔╝╚██████╔╝
╚═════╝ ╚═════╝  ╚═════╝ 
                         
Search and advanced search in DuckDuckGo
========================================

Perform searches and advanced searches on DuckDuckGo from the command line.

FEATURES:
  - Search query: Set the search query with the --query or -q option.
  - Search operators: Use the --operators or -o option to set search operators.
  - Safe search: Enable safe search with the --safe option.
  - Output format: Set the output format (list or detailed) with the --format option.
  - Result limit: Limit the number of results with the --limit option.
  - User agent: Set the user agent for the HTTP client with the --user-agent option.
  - Cookie: Set the cookie for the HTTP client with the --cookie option.
  - Proxy: Set the proxy for the HTTP client with the --proxy option.
  - Backend: Choose the backend used for search (e.g. auto, lite, images, news)
    with the --backend option.
  - Verbose mode: Show debug messages with the --verbose or -v option.

USAGE:
  ddg [OPTIONS]

EXAMPLES:
  - Perform a basic search:
    ddg --query "rust lang"

  - Use search operators:
    ddg --query "rust lang" --operators "+tutorial +beginner"

  - Enable safe search:
    ddg --query "rust lang" --safe

  - Set the output format to detailed:
    ddg --query "rust lang" --format

  - Limit the number of results to 10:
    ddg --query "rust lang" --limit 10

  - Set user agent:
    ddg --query "rust lang" --user-agent "chrome"

  - Set cookie for subsequent requests:
    ddg --query "rust lang" --cookie

  - Set proxy:
    ddg --query "rust lang" --proxy "socks5://192.168.1.1:9000"

  - Use a specific backend:
    ddg --query "rust lang" --backend news

  - Enable verbose mode:
    ddg --query "rust lang" --verbose

For more information, visit: https://github.com/wiseaidotdev/duckduckgo
"#
)]
pub struct Cli {
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Sets the search query.
    #[arg(short = 'q', long = "query")]
    pub query: String,

    /// Sets the search operators.
    #[arg(short = 'o', long = "operators", default_value_t = String::from(""))]
    pub operators: String,

    /// Enable safe search.
    #[arg(short = 's', long = "safe", default_value_t = false)]
    pub safe: bool,

    /// Sets the output format (`false` for list or `true` for detailed).
    #[arg(short = 'f', long = "format", default_value_t = false)]
    pub format: bool,

    /// Limits the number of results (default is 10).
    #[arg(short = 'l', long = "limit", default_value_t = 10)]
    pub limit: usize,

    /// Sets the user agent for the HTTP client.
    #[arg(short = 'u', long = "user-agent", default_value_t = String::from("firefox"))]
    pub user_agent: String,

    /// Sets the cookie for subsequent HTTP requests.
    #[arg(short = 'c', long = "cookie", default_value_t = true)]
    pub cookie: bool,

    /// Sets the proxy for the HTTP client (e.g. "socks5://192.168.1.1:9000").
    #[arg(short = 'p', long = "proxy", default_value_t = String::from(""))]
    pub proxy: String,

    /// Sets the backend to use.
    #[arg(short = 'b', long = "backend", value_enum, default_value_t = Backend::Auto)]
    pub backend: Backend,
}
