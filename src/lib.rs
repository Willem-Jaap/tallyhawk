//! # Tallyhawk
//! 
//! A sharp-eyed CLI tool for gathering project statistics.
//! 
//! This crate provides functionality to analyze source code projects,
//! count lines of code, identify file types, and generate statistics
//! in various output formats.

pub mod commands;
pub mod error;
pub mod stats;
pub mod types;
pub mod utils;

pub use error::{TallyhawkError, Result};
pub use types::OutputFormat;

pub use commands::count::{CountConfig, run as count};
pub use stats::counter::ProjectStats;
pub use stats::file_types::FileType;