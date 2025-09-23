use crate::stats::counter::ProjectStats;
use crate::types::OutputFormat;
use crate::utils::output::OutputFormatter;
use std::path::PathBuf;

pub struct CountConfig {
    pub path: PathBuf,
    pub include_hidden: bool,
    pub output_format: OutputFormat,
    pub respect_gitignore: bool,
    pub include_blank_lines: bool,
    pub include_comments: bool,
}

pub fn run(config: CountConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("🦅 Tallyhawk surveying: {}", config.path.display());

    let mut stats = ProjectStats::new();
    stats.scan_directory(&config.path, &config)?;

    let formatter = OutputFormatter::new(config.output_format);
    formatter.display(&stats)?;

    Ok(())
}
