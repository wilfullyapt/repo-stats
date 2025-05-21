mod analyzer;
mod stats;
mod utils;

use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use std::path::Path;
use colored::*;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
enum Language {
    Python,
    Rust,
    C,
    Cpp,
}

#[derive(Parser, Debug)]
#[clap(about = "A CLI tool to analyze code statistics for various programming languages")]
struct Args {
    /// Path to the project directory containing an 'src' subdirectory
    project_dir: String,
    
    /// Programming language to analyze (python, rust, c, cpp)
    language: Language,
}

fn main() {
    let args = Args::parse();
    let src_dir = Path::new(&args.project_dir);
//  let src_dir = project_dir.join("src");

    if !src_dir.is_dir() {
        eprintln!("{}", "Error: 'src' directory not found in the specified path".red());
        std::process::exit(1);
    }

    let extension = match args.language {
        Language::Python => "py",
        Language::Rust => "rs",
        Language::C => "c",
        Language::Cpp => "cpp",
    };

    let files = utils::get_files_with_extension(&src_dir, extension);

    if files.is_empty() {
        println!("{}", format!("No .{} files found in src directory", extension).yellow());
        std::process::exit(0);
    }

    println!("{}", "Analyzed files:".blue().bold());
    for file in &files {
        println!("{}", format!(" - {}", file).green());
    }

    let mut stats = HashMap::new();
    stats.insert("filecount".to_string(), files.len());

    for file in files {
        let path = src_dir.join(file);
        analyzer::analyze_file(&path, &args.language, &mut stats);
    }

    stats::print_stats(&stats, &args.language);
}
