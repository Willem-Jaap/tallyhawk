//! # TallyHawk
//!
//! A sharp-eyed CLI tool for gathering project statistics.
//!
//! This crate provides functionality to analyze source code projects,
//! count lines of code, identify file types, and generate statistics
//! in various output formats.

pub mod commands;
pub mod stats;
pub mod types;
pub mod utils;

pub use types::OutputFormat;

pub use commands::count::{run as count, CountConfig};
pub use stats::counter::ProjectStats;
pub use stats::file_types::FileType;
