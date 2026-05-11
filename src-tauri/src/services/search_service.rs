use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchMatch {
    pub line: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub name: String,
    pub matches: Vec<SearchMatch>,
    pub is_filename_match: bool,
}

const MAX_RESULTS: usize = 50;
const MAX_MATCHES_PER_FILE: usize = 5;
const MAX_LINE_LENGTH: usize = 120;

pub struct SearchService;

impl SearchService {
    pub fn search(workspace_path: &Path, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return vec![];
        }

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        Self::search_recursive(workspace_path, &query_lower, &mut results);
        results
    }

    fn search_recursive(dir: &Path, query: &str, results: &mut Vec<SearchResult>) {
        if results.len() >= MAX_RESULTS {
            return;
        }

        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            if results.len() >= MAX_RESULTS {
                break;
            }

            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_lowercase();

            if name.starts_with('.') || name == "knotic.json" || name == "node_modules" {
                continue;
            }

            if path.is_dir() {
                Self::search_recursive(&path, query, results);
            } else {
                let original_name = entry.file_name().to_string_lossy().into_owned();
                let is_filename_match = name.contains(query);
                let mut file_matches = Vec::new();

                if let Ok(file) = fs::File::open(&path) {
                    let reader = BufReader::new(file);
                    for (line_num, line) in reader.lines().flatten().enumerate() {
                        if file_matches.len() >= MAX_MATCHES_PER_FILE {
                            break;
                        }
                        if line.to_lowercase().contains(query) {
                            let content = if line.len() > MAX_LINE_LENGTH {
                                let lower = line.to_lowercase();
                                if let Some(pos) = lower.find(query) {
                                    let start = pos.saturating_sub(40);
                                    let end = (pos + query.len() + 40).min(line.len());
                                    let mut snippet = String::from("...");
                                    snippet.push_str(&line[start..end]);
                                    snippet.push_str("...");
                                    snippet
                                } else {
                                    line[..MAX_LINE_LENGTH].to_string()
                                }
                            } else {
                                line.clone()
                            };

                            file_matches.push(SearchMatch {
                                line: line_num + 1,
                                content,
                            });
                        }
                    }
                }

                if is_filename_match || !file_matches.is_empty() {
                    results.push(SearchResult {
                        path,
                        name: original_name,
                        matches: file_matches,
                        is_filename_match,
                    });
                }
            }
        }
    }
}
