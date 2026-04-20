// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::Result;
#[cfg(feature = "cli")]
use duckduckgo::app::run_cli_entry;

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<()> {
    run_cli_entry(std::env::args().collect()).await
}

#[cfg(not(feature = "cli"))]
fn main() -> Result<()> {
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
