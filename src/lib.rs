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
