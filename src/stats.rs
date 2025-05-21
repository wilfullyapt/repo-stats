use colored::*;
use std::collections::HashMap;
use crate::Language;

pub fn print_stats(stats: &HashMap<String, usize>, language: &Language) {
    println!("{}", "\n=========================".blue());
    println!("{}", format!("   Files         :   {}", stats.get("filecount").unwrap_or(&0)).yellow());

    if let Some(total_lines) = stats.get("total_lines") {
        println!("{}", format!("   Lines         :   {}", total_lines).yellow());
    }

    // Language-specific keywords
    match language {
        Language::Python => {
            println!("{}", format!("   Classes       :   {}", stats.get("classes").unwrap_or(&0)).yellow());
            println!("{}", format!("   Methods       :   {}", stats.get("methods").unwrap_or(&0)).yellow());
        }
        Language::Rust => {
            println!("{}", format!("   Structs       :   {}", stats.get("structs").unwrap_or(&0)).yellow());
            println!("{}", format!("   Functions     :   {}", stats.get("functions").unwrap_or(&0)).yellow());
        }
        Language::C | Language::Cpp => {
            // C/C++ might not track specific keywords like classes, so skip or adjust
        }
    }

    println!("{}", format!("   Ifs           :   {}", stats.get("ifs").unwrap_or(&0)).yellow());
    println!("{}", format!("   Comments      :   {}", stats.get("comment_lines").unwrap_or(&0)).yellow());
    println!("{}", format!("   Blank Lines   :   {}", stats.get("blank_lines").unwrap_or(&0)).yellow());
    println!("{}", format!("   Code Lines    :   {}", stats.get("code_lines").unwrap_or(&0)).yellow());

    // Calculate derived stats
    let total_lines = *stats.get("total_lines").unwrap_or(&0);
    let comment_lines = *stats.get("comment_lines").unwrap_or(&0);
    let blank_lines = *stats.get("blank_lines").unwrap_or(&0);
    println!("{}", format!("   Sans Comments :   {}", total_lines.saturating_sub(comment_lines)).yellow());
    println!("{}", format!("   Sans Blanks   :   {}", total_lines.saturating_sub(blank_lines)).yellow());
    println!("{}", format!("   Sans Both     :   {}", total_lines.saturating_sub(comment_lines + blank_lines)).yellow());
    println!("{}", "=========================".blue());
}
