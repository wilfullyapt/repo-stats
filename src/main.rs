mod analyzer;
mod cache;
mod stats;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::path::Path;
use colored::*;
use std::env;
use std::fs;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
enum Language {
    Python,
    Rust,
    C,
    Cpp,
}

#[derive(Parser)]
#[clap(about = "A CLI tool to analyze code statistics for various programming languages")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Analyze code statistics in a directory
    Analyze {
        /// Path to the project directory
        project_dir: String,
        /// Programming language to analyze (python, rust, c, cpp)
        language: Language,
    },
    /// Install repostats to ~/.local/bin
    Install,
    /// Uninstall repostats from ~/.local/bin
    Uninstall,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Analyze { project_dir, language } => {
            let src_dir = Path::new(&project_dir);
            if !src_dir.is_dir() {
                eprintln!("{}", "Error: Specified directory not found".red());
                std::process::exit(1);
            }

            let extension = match language {
                Language::Python => "py",
                Language::Rust => "rs",
                Language::C => "c",
                Language::Cpp => "cpp",
            };

            let lang_str = match language {
                Language::Python => "python",
                Language::Rust => "rust",
                Language::C => "c",
                Language::Cpp => "cpp",
            };

            let files = utils::get_files_with_extension(src_dir, extension);

            if files.is_empty() {
                println!("{}", format!("No .{} files found in the specified directory", extension).yellow());
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
                analyzer::analyze_file(&path, &language, &mut stats);
            }

            let prev_stats = cache::load_cache(src_dir, lang_str);
            cache::save_cache(src_dir, lang_str, &stats);
            stats::print_stats(&stats, &prev_stats, &language);
        }
        Command::Install => install(),
        Command::Uninstall => uninstall(),
    }
}

fn install() {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let local_bin = Path::new(&home_dir).join(".local").join("bin");
    let local_bin_str = local_bin.to_str().expect("Invalid path");

    // Check if ~/.local/bin is in $PATH
    let path_env = env::var("PATH").unwrap_or_default();
    let path_dirs: Vec<&str> = path_env.split(':').collect();
    if !path_dirs.contains(&local_bin_str) {
        println!(
            "Warning: {} is not in your $PATH. Add it to use repostats globally.",
            local_bin.display()
        );
    }

    // Create ~/.local/bin if it doesn’t exist
    if !local_bin.exists() {
        if let Err(e) = fs::create_dir_all(&local_bin) {
            eprintln!("Failed to create {}: {}", local_bin.display(), e);
            std::process::exit(1);
        }
    }

    // Copy the current executable to ~/.local/bin/repostats
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let dest = local_bin.join("repostats");
    if let Err(e) = fs::copy(&current_exe, &dest) {
        eprintln!("Failed to install repostats: {}", e);
        std::process::exit(1);
    }

    println!("repostats installed successfully to {}", dest.display());
}

fn uninstall() {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let local_bin = Path::new(&home_dir).join(".local").join("bin");
    let dest = local_bin.join("repostats");

    if dest.exists() {
        if let Err(e) = fs::remove_file(&dest) {
            eprintln!("Failed to uninstall repostats: {}", e);
            std::process::exit(1);
        }
        println!("repostats uninstalled successfully");
    } else {
        println!("repostats is not installed in {}", local_bin.display());
    }
}
