use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileType {
    pub language: String,
    pub is_binary: bool,
    pub comment_patterns: Vec<&'static str>,
}

impl FileType {
    /// Detect file type and language from file path extension.
    /// Maps common extensions to languages with their comment patterns.
    pub fn from_path(path: &Path) -> Self {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            // Rust
            "rs" => FileType {
                language: "Rust".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // JavaScript/TypeScript
            "js" | "jsx" | "mjs" => FileType {
                language: "JavaScript".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "ts" | "tsx" => FileType {
                language: "TypeScript".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // Python
            "py" | "pyx" | "pyi" => FileType {
                language: "Python".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },

            // C/C++
            "c" | "h" => FileType {
                language: "C".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => FileType {
                language: "C++".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // Java
            "java" => FileType {
                language: "Java".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // Go
            "go" => FileType {
                language: "Go".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // Shell
            "sh" | "bash" | "zsh" | "fish" => FileType {
                language: "Shell".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },

            // Web languages
            "html" | "htm" => FileType {
                language: "HTML".to_string(),
                is_binary: false,
                comment_patterns: vec!["<!--"],
            },
            "css" => FileType {
                language: "CSS".to_string(),
                is_binary: false,
                comment_patterns: vec!["/*"],
            },
            "scss" | "sass" => FileType {
                language: "Sass".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },

            // Config files
            "json" => FileType {
                language: "JSON".to_string(),
                is_binary: false,
                comment_patterns: vec![], // JSON doesn't support comments
            },
            "yaml" | "yml" => FileType {
                language: "YAML".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },
            "toml" => FileType {
                language: "TOML".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },
            "xml" => FileType {
                language: "XML".to_string(),
                is_binary: false,
                comment_patterns: vec!["<!--"],
            },

            // Markup
            "md" | "markdown" => FileType {
                language: "Markdown".to_string(),
                is_binary: false,
                comment_patterns: vec!["<!--"],
            },
            "rst" => FileType {
                language: "reStructuredText".to_string(),
                is_binary: false,
                comment_patterns: vec![".."],
            },

            // Other languages
            "rb" => FileType {
                language: "Ruby".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },
            "php" => FileType {
                language: "PHP".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*", "#"],
            },
            "swift" => FileType {
                language: "Swift".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "kt" | "kts" => FileType {
                language: "Kotlin".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "cs" => FileType {
                language: "C#".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "dart" => FileType {
                language: "Dart".to_string(),
                is_binary: false,
                comment_patterns: vec!["//", "/*"],
            },
            "r" => FileType {
                language: "R".to_string(),
                is_binary: false,
                comment_patterns: vec!["#"],
            },
            "sql" => FileType {
                language: "SQL".to_string(),
                is_binary: false,
                comment_patterns: vec!["--", "/*"],
            },

            // Binary files
            "exe" | "dll" | "so" | "dylib" | "a" | "lib" => FileType {
                language: "Binary".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" => FileType {
                language: "Image".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },
            "mp3" | "wav" | "ogg" | "flac" | "aac" => FileType {
                language: "Audio".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => FileType {
                language: "Video".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => FileType {
                language: "Archive".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => FileType {
                language: "Document".to_string(),
                is_binary: true,
                comment_patterns: vec![],
            },

            // Default for unknown files
            _ => {
                // Try to guess if it's binary by checking for common text file patterns
                let is_likely_binary = self::is_likely_binary_extension(&extension);

                FileType {
                    language: if is_likely_binary { "Binary" } else { "Text" }.to_string(),
                    is_binary: is_likely_binary,
                    comment_patterns: vec!["#", "//"], // Default comment patterns
                }
            }
        }
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn is_binary(&self) -> bool {
        self.is_binary
    }

    /// Check if a line is a comment based on language-specific patterns.
    /// Supports single-line comments like //, #, --, etc.
    pub fn is_comment_line(&self, line: &str) -> bool {
        let trimmed = line.trim();

        for pattern in &self.comment_patterns {
            if trimmed.starts_with(pattern) {
                return true;
            }
        }

        false
    }
}

fn is_likely_binary_extension(ext: &str) -> bool {
    matches!(
        ext,
        "bin"
            | "dat"
            | "db"
            | "sqlite"
            | "sqlite3"
            | "lock"
            | "log"
            | "tmp"
            | "temp"
            | "cache"
            | "o"
            | "obj"
            | "pyc"
            | "class"
            | "jar"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_rust_file_detection() {
        let path = Path::new("test.rs");
        let file_type = FileType::from_path(path);
        assert_eq!(file_type.language, "Rust");
        assert!(!file_type.is_binary);
        assert!(file_type.comment_patterns.contains(&"//"));
    }

    #[test]
    fn test_python_file_detection() {
        let path = Path::new("script.py");
        let file_type = FileType::from_path(path);
        assert_eq!(file_type.language, "Python");
        assert!(!file_type.is_binary);
        assert!(file_type.comment_patterns.contains(&"#"));
    }

    #[test]
    fn test_binary_file_detection() {
        let path = Path::new("program.exe");
        let file_type = FileType::from_path(path);
        assert_eq!(file_type.language, "Binary");
        assert!(file_type.is_binary);
    }

    #[test]
    fn test_no_extension() {
        let path = Path::new("README");
        let file_type = FileType::from_path(path);
        assert_eq!(file_type.language, "Text");
        assert!(!file_type.is_binary);
    }

    #[test]
    fn test_is_likely_binary_extension() {
        assert!(is_likely_binary_extension("bin"));
        assert!(is_likely_binary_extension("pyc"));
        assert!(!is_likely_binary_extension("txt"));
        assert!(!is_likely_binary_extension("rs"));
    }
}
