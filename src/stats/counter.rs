use crate::commands::count::CountConfig;
use crate::stats::file_types::FileType;
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_code_lines: usize,
    pub total_comment_lines: usize,
    pub total_blank_lines: usize,
    pub file_types: HashMap<String, FileTypeStats>,
    pub total_size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub count: usize,
    pub lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub size_bytes: u64,
}

impl Default for ProjectStats {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectStats {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            total_lines: 0,
            total_code_lines: 0,
            total_comment_lines: 0,
            total_blank_lines: 0,
            file_types: HashMap::new(),
            total_size_bytes: 0,
        }
    }

    /// Recursively scan directory and collect file statistics.
    /// Respects .gitignore files and hidden file preferences.
    pub fn scan_directory(
        &mut self,
        path: &Path,
        config: &CountConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut builder = WalkBuilder::new(path);

        builder
            .hidden(!config.include_hidden)
            .git_ignore(config.respect_gitignore)
            .git_exclude(config.respect_gitignore);

        for result in builder.build() {
            let entry = result?;

            if entry.file_type().is_some_and(|ft| ft.is_file()) {
                self.process_file(entry.path(), config)?;
            }
        }

        Ok(())
    }

    /// Process a single file: count lines, detect type, measure size.
    /// Binary files are tracked but not analyzed for line content.
    fn process_file(
        &mut self,
        path: &Path,
        config: &CountConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();

        let file_type = FileType::from_path(path);

        // Skip binary files for line counting
        if file_type.is_binary() {
            self.add_binary_file(&file_type, file_size);
            return Ok(());
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                // If we can't read as UTF-8, treat as binary
                if e.kind() == std::io::ErrorKind::InvalidData {
                    self.add_binary_file(&file_type, file_size);
                    return Ok(());
                }
                // Re-throw other IO errors
                return Err(e.into());
            }
        };
        let line_stats = self.analyze_lines(&content, &file_type, config);

        self.total_files += 1;
        self.total_lines += line_stats.total;
        self.total_code_lines += line_stats.code;
        self.total_comment_lines += line_stats.comments;
        self.total_blank_lines += line_stats.blank;
        self.total_size_bytes += file_size;

        let language_key = file_type.language().to_string();
        let entry = self
            .file_types
            .entry(language_key)
            .or_insert(FileTypeStats {
                count: 0,
                lines: 0,
                code_lines: 0,
                comment_lines: 0,
                blank_lines: 0,
                size_bytes: 0,
            });

        entry.count += 1;
        entry.lines += line_stats.total;
        entry.code_lines += line_stats.code;
        entry.comment_lines += line_stats.comments;
        entry.blank_lines += line_stats.blank;
        entry.size_bytes += file_size;

        Ok(())
    }

    /// Track binary files (images, executables, etc.) without line analysis.
    fn add_binary_file(&mut self, file_type: &FileType, size: u64) {
        self.total_files += 1;
        self.total_size_bytes += size;

        let language_key = file_type.language().to_string();
        let entry = self
            .file_types
            .entry(language_key)
            .or_insert(FileTypeStats {
                count: 0,
                lines: 0,
                code_lines: 0,
                comment_lines: 0,
                blank_lines: 0,
                size_bytes: 0,
            });

        entry.count += 1;
        entry.size_bytes += size;
    }

    /// Analyze file content line by line: categorize as code, comments, or blanks.
    fn analyze_lines(
        &self,
        content: &str,
        file_type: &FileType,
        config: &CountConfig,
    ) -> LineStats {
        let lines: Vec<&str> = content.lines().collect();
        let mut stats = LineStats {
            total: lines.len(),
            code: 0,
            comments: 0,
            blank: 0,
        };

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                stats.blank += 1;
            } else if file_type.is_comment_line(trimmed) {
                stats.comments += 1;
            } else {
                stats.code += 1;
            }
        }

        if !config.include_blank_lines {
            stats.total -= stats.blank;
        }
        if !config.include_comments {
            stats.total -= stats.comments;
        }

        stats
    }
}

struct LineStats {
    total: usize,
    code: usize,
    comments: usize,
    blank: usize,
}
