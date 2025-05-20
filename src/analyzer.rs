use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use crate::Language;

pub fn analyze_file(path: &Path, language: &Language, stats: &mut HashMap<String, usize>) {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().filter_map(|l| l.ok()) {
            *stats.entry("total_lines".to_string()).or_insert(0) += 1;
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                *stats.entry("blank_lines".to_string()).or_insert(0) += 1;
            } else if is_comment(trimmed, language) {
                *stats.entry("comment_lines".to_string()).or_insert(0) += 1;
            } else if let Some(keyword) = get_keyword(trimmed, language) {
                *stats.entry(keyword.to_string()).or_insert(0) += 1;
                *stats.entry("code_lines".to_string()).or_insert(0) += 1;
            } else {
                *stats.entry("code_lines".to_string()).or_insert(0) += 1;
            }
        }
    } else {
        eprintln!("Warning: Could not read file {}", path.display());
    }
}

fn is_comment(line: &str, language: &Language) -> bool {
    match language {
        Language::Python => line.starts_with('#'),
        Language::Rust | Language::C | Language::Cpp => line.starts_with("//"),
    }
}

fn get_keyword(line: &str, language: &Language) -> Option<&'static str> {
    match language {
        Language::Python => {
            if line.starts_with("class ") { Some("classes") }
            else if line.starts_with("def ") { Some("methods") }
            else if line.starts_with("if ") { Some("ifs") }
            else { None }
        }
        Language::Rust => {
            if line.starts_with("struct ") { Some("structs") }
            else if line.starts_with("fn ") { Some("functions") }
            else if line.starts_with("if ") { Some("ifs") }
            else { None }
        }
        Language::C | Language::Cpp => {
            if line.starts_with("if (") { Some("ifs") }
            else { None }
        }
    }
}
