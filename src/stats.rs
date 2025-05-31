use colored::*;
use std::collections::HashMap;
use crate::Language;

pub fn print_stats(stats: &HashMap<String, usize>, prev_stats: &Option<HashMap<String, usize>>, language: &Language) {
    println!("{}", "\n=========================".blue());

    let print_stat = |key: &str, label: &str| {
        let current = stats.get(key).unwrap_or(&0);
        let diff = prev_stats.as_ref().map(|prev| {
            let prev_val = prev.get(key).unwrap_or(&0);
            (*current as i64) - (*prev_val as i64)
        });
        let diff_str = match diff {
            Some(d) if d > 0 => format!(" +{}", d).green(),
            Some(d) if d < 0 => format!(" {}", d).red(),
            Some(_) => " 0".white(),
            None => "".normal(), // Fixed: Now returns ColoredString
        };
        println!("{}", format!("   {:<13} :   {:<8} {}", label, current, diff_str).yellow());
    };

    print_stat("filecount", "Files");
    print_stat("total_lines", "Lines");

    // Language-specific keywords
    match language {
        Language::Python => {
            print_stat("classes", "Classes");
            print_stat("methods", "Methods");
        }
        Language::Rust => {
            print_stat("structs", "Structs");
            print_stat("functions", "Functions");
        }
        Language::C | Language::Cpp => {
            // C/C++ might not track specific keywords like classes, so skip or adjust
        }
    }

    print_stat("ifs", "Ifs");
    print_stat("comment_lines", "Comments");
    print_stat("blank_lines", "Blank Lines");
    print_stat("code_lines", "Code Lines");

    // Calculate derived stats
    let total_lines = *stats.get("total_lines").unwrap_or(&0);
    let comment_lines = *stats.get("comment_lines").unwrap_or(&0);
    let blank_lines = *stats.get("blank_lines").unwrap_or(&0);

    let sans_comments = total_lines.saturating_sub(comment_lines);
    let sans_comments_diff = prev_stats.as_ref().map(|prev| {
        let prev_total = *prev.get("total_lines").unwrap_or(&0);
        let prev_comments = *prev.get("comment_lines").unwrap_or(&0);
        (sans_comments as i64) - (prev_total.saturating_sub(prev_comments) as i64)
    });
    let sans_comments_diff_str = match sans_comments_diff {
        Some(d) if d > 0 => format!(" +{}", d).green(),
        Some(d) if d < 0 => format!(" {}", d).red(),
        Some(_) => " 0".white(),
        None => "".normal(), // Fixed: Now returns ColoredString
    };
    println!("{}", format!("   {:<13} :   {:<8} {}", "Sans Comments", sans_comments, sans_comments_diff_str).yellow());

    let sans_blanks = total_lines.saturating_sub(blank_lines);
    let sans_blanks_diff = prev_stats.as_ref().map(|prev| {
        let prev_total = *prev.get("total_lines").unwrap_or(&0);
        let prev_blanks = *prev.get("blank_lines").unwrap_or(&0);
        (sans_blanks as i64) - (prev_total.saturating_sub(prev_blanks) as i64)
    });
    let sans_blanks_diff_str = match sans_blanks_diff {
        Some(d) if d > 0 => format!(" +{}", d).green(),
        Some(d) if d < 0 => format!(" {}", d).red(),
        Some(_) => " 0".white(),
        None => "".normal(), // Fixed: Now returns ColoredString
    };
    println!("{}", format!("   {:<13} :   {:<8} {}", "Sans Blanks", sans_blanks, sans_blanks_diff_str).yellow());

    let sans_both = total_lines.saturating_sub(comment_lines + blank_lines);
    let sans_both_diff = prev_stats.as_ref().map(|prev| {
        let prev_total = *prev.get("total_lines").unwrap_or(&0);
        let prev_comments = *prev.get("comment_lines").unwrap_or(&0);
        let prev_blanks = *prev.get("blank_lines").unwrap_or(&0);
        (sans_both as i64) - (prev_total.saturating_sub(prev_comments + prev_blanks) as i64)
    });
    let sans_both_diff_str = match sans_both_diff {
        Some(d) if d > 0 => format!(" +{}", d).green(),
        Some(d) if d < 0 => format!(" {}", d).red(),
        Some(_) => " 0".white(),
        None => "".normal(), // Fixed: Now returns ColoredString
    };
    println!("{}", format!("   {:<13} :   {:<8} {}", "Sans Both", sans_both, sans_both_diff_str).yellow());

    println!("{}", "=========================".blue());
}
