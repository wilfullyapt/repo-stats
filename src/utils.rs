use std::path::Path;
use walkdir::WalkDir;

pub fn get_files_with_extension(src_dir: &Path, extension: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && 
           entry.path().extension().and_then(|s| s.to_str()) == Some(extension) {
            if let Ok(rel_path) = entry.path().strip_prefix(src_dir) {
                files.push(rel_path.to_string_lossy().into_owned());
            }
        }
    }
    files
}
