use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    project_dir: String,
}

struct Stats {
    filecount: usize,
    linestotal: usize,
    classlines: usize,
    methodlines: usize,
    iflines: usize,
    commentlines: usize,
    blanklines: usize,
    codelines: usize,
}

fn analyze_file(path: &Path, stats: &mut Stats) {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim_start();
                if trimmed.is_empty() {
                    stats.blanklines += 1;
                } else if trimmed.starts_with('#') {
                    stats.commentlines += 1;
                } else if trimmed.starts_with("def ") {
                    stats.methodlines += 1;
                } else if trimmed.starts_with("class ") {
                    stats.classlines += 1;
                } else if trimmed.starts_with("if ") {
                    stats.iflines += 1;
                } else {
                    stats.codelines += 1;
                }
                stats.linestotal += 1;
            }
        }
    } else {
        eprintln!("Warning: Could not read file {}", path.display());
    }
}

fn get_python_files(src_dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("py") {
            if let Ok(rel_path) = entry.path().strip_prefix(src_dir) {
                files.push(rel_path.to_string_lossy().into_owned());
            }
        }
    }
    files
}

fn main() {
    let args = Args::parse();
    let project_dir = Path::new(&args.project_dir);
    let src_dir = project_dir.join("src");
    
    if !src_dir.is_dir() {
        eprintln!("{}", "Error: src directory not found".red());
        std::process::exit(1);
    }
    
    let python_files = get_python_files(&src_dir);
    
    if python_files.is_empty() {
        println!("{}", "No Python files found in src directory.".yellow());
        std::process::exit(0);
    }
    
    println!("{}", "Analyzed Python files:".blue());
    for file in &python_files {
        println!("{}", format!(" - {}", file).green());
    }
    
    let mut stats = Stats {
        filecount: python_files.len(),
        linestotal: 0,
        classlines: 0,
        methodlines: 0,
        iflines: 0,
        commentlines: 0,
        blanklines: 0,
        codelines: 0,
    };
    
    for file in &python_files {
        let path = src_dir.join(file);
        analyze_file(&path, &mut stats);
    }
    
    println!("{}", "\n=========================".blue());
    println!("{}", format!("   Files         :   {}", stats.filecount).yellow());
    println!("{}", format!("   Lines         :   {}", stats.linestotal).yellow());
    println!("{}", format!("   Classes       :   {}", stats.classlines).yellow());
    println!("{}", format!("   Methods       :   {}", stats.methodlines).yellow());
    println!("{}", format!("   Ifs           :   {}", stats.iflines).yellow());
    println!("{}", format!("   Comments      :   {}", stats.commentlines).yellow());
    println!("{}", format!("   Blank Lines   :   {}", stats.blanklines).yellow());
    println!("{}", format!("   Code Lines    :   {}", stats.codelines).yellow());
    println!("{}", format!("   Sans Comments :   {}", stats.linestotal - stats.commentlines).yellow());
    println!("{}", format!("   Sans Blanks   :   {}", stats.linestotal - stats.blanklines).yellow());
    println!("{}", format!("   Sans Both     :   {}", stats.linestotal - stats.commentlines - stats.blanklines).yellow());
    println!("{}", "=========================".blue());
}
