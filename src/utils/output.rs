use crate::stats::counter::ProjectStats;
use crate::types::OutputFormat;
use colored::*;

pub struct OutputFormatter {
    format: OutputFormat,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Display statistics in the specified format (table, JSON, or CSV).
    pub fn display(&self, stats: &ProjectStats) -> Result<(), Box<dyn std::error::Error>> {
        match self.format {
            OutputFormat::Table => self.display_table(stats),
            OutputFormat::Json => self.display_json(stats),
            OutputFormat::Csv => self.display_csv(stats),
        }
    }

    fn display_table(&self, stats: &ProjectStats) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "🦅 Tallyhawk survey results".bold().cyan());
        println!("{}", "═".repeat(50).bright_cyan());

        println!("\n{}", "📊 Project overview".bold().yellow());
        println!(
            "{}  {}",
            "Total Files:".bright_white(),
            stats.total_files.to_string().green().bold()
        );
        println!(
            "{}  {}",
            "Total Lines:".bright_white(),
            stats.total_lines.to_string().green().bold()
        );
        println!(
            "{}  {}",
            "Code Lines:".bright_white(),
            stats.total_code_lines.to_string().blue().bold()
        );
        println!(
            "{}  {}",
            "Comment Lines:".bright_white(),
            stats.total_comment_lines.to_string().yellow().bold()
        );
        println!(
            "{}  {}",
            "Blank Lines:".bright_white(),
            stats.total_blank_lines.to_string().bright_black().bold()
        );
        println!(
            "{}  {}",
            "Total Size:".bright_white(),
            format_bytes(stats.total_size_bytes).magenta().bold()
        );

        if !stats.file_types.is_empty() {
            println!("\n{}", "📁 File Types Breakdown".bold().yellow());
            println!("{}", "─".repeat(80).bright_yellow());

            println!(
                "{:<15} {:>8} {:>10} {:>10} {:>12} {:>12}",
                "Language".bold().bright_white(),
                "Files".bold().bright_white(),
                "Lines".bold().bright_white(),
                "Code".bold().bright_white(),
                "Comments".bold().bright_white(),
                "Size".bold().bright_white()
            );
            println!("{}", "─".repeat(80).bright_black());

            // Sort by line count (descending)
            let mut sorted_types: Vec<_> = stats.file_types.iter().collect();
            sorted_types.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

            for (language, file_stats) in sorted_types {
                let percentage = if stats.total_lines > 0 {
                    (file_stats.lines as f64 / stats.total_lines as f64) * 100.0
                } else {
                    0.0
                };

                println!(
                    "{:<15} {:>8} {:>10} {:>10} {:>12} {:>12}",
                    self.colorize_language(language),
                    file_stats.count.to_string().bright_white(),
                    format!("{} ({:.1}%)", file_stats.lines, percentage).green(),
                    file_stats.code_lines.to_string().blue(),
                    file_stats.comment_lines.to_string().yellow(),
                    format_bytes(file_stats.size_bytes).magenta()
                );
            }
        }

        if stats.file_types.len() > 3 {
            println!("\n{}", "🏆 Top Languages by Lines".bold().yellow());
            let mut top_types: Vec<_> = stats.file_types.iter().collect();
            top_types.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

            for (i, (language, file_stats)) in top_types.iter().take(5).enumerate() {
                let medal = match i {
                    0 => "🥇",
                    1 => "🥈",
                    2 => "🥉",
                    _ => "  ",
                };
                let percentage = (file_stats.lines as f64 / stats.total_lines as f64) * 100.0;
                println!(
                    "{} {} - {} lines ({:.1}%)",
                    medal,
                    language.bold(),
                    file_stats.lines.to_string().green().bold(),
                    percentage
                );
            }
        }

        println!("\n{}", "─".repeat(50).bright_cyan());
        println!("{}", "Survey complete! 🦅✨".bright_cyan().bold());

        Ok(())
    }

    fn display_json(&self, stats: &ProjectStats) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(stats)?;
        println!("{}", json);
        Ok(())
    }

    fn display_csv(&self, stats: &ProjectStats) -> Result<(), Box<dyn std::error::Error>> {
        println!("language,extension,files,lines,code_lines,comment_lines,blank_lines,size_bytes");

        for (language, file_stats) in &stats.file_types {
            println!(
                "{},multiple,{},{},{},{},{},{}",
                language,
                file_stats.count,
                file_stats.lines,
                file_stats.code_lines,
                file_stats.comment_lines,
                file_stats.blank_lines,
                file_stats.size_bytes
            );
        }

        println!(
            "TOTAL,ALL,{},{},{},{},{},{}",
            stats.total_files,
            stats.total_lines,
            stats.total_code_lines,
            stats.total_comment_lines,
            stats.total_blank_lines,
            stats.total_size_bytes
        );

        Ok(())
    }

    fn colorize_language(&self, language: &str) -> ColoredString {
        match language {
            "Rust" => language.red().bold(),
            "JavaScript" | "TypeScript" => language.yellow().bold(),
            "Python" => language.blue().bold(),
            "C" | "C++" => language.cyan().bold(),
            "Java" => language.bright_red().bold(),
            "Go" => language.bright_cyan().bold(),
            "Ruby" => language.red(),
            "PHP" => language.purple().bold(),
            "Swift" => language.bright_red(),
            "HTML" => language.bright_yellow(),
            "CSS" | "Sass" => language.blue(),
            "Markdown" => language.bright_white(),
            "Shell" => language.green().bold(),
            "JSON" | "YAML" | "TOML" | "XML" => language.bright_magenta(),
            _ => language.normal(),
        }
    }
}

/// Convert bytes to human-readable format (B, KB, MB, GB, TB).
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }
}
