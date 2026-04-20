// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/wiseaidotdev/duckduckgo/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/wiseaidotdev/duckduckgo/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod browser;
#[cfg(feature = "cli")]
pub mod cli;
pub mod colors;
pub mod icon;
pub mod params;
pub mod response;
pub mod topic;
pub mod user_agents;

#[cfg(feature = "cli")]
pub mod app;

#[cfg(all(feature = "python", not(feature = "rust-binary")))]
pub mod python;

#[cfg(all(feature = "python", not(feature = "rust-binary")))]
use pyo3::prelude::*;

#[cfg(all(feature = "python", not(feature = "rust-binary")))]
#[pyfunction]
fn run_cli(args: Vec<String>) -> PyResult<()> {
    tokio::runtime::Runtime::new()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
        .block_on(crate::app::run_cli_entry(args))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[cfg(all(feature = "python", not(feature = "rust-binary")))]
#[pymodule]
fn _ddg(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    crate::python::register_python_module(py, m)?;
    Ok(())
}

#[cfg(all(feature = "node", not(feature = "rust-binary")))]
pub mod node;

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
