pub mod ast_ops;

pub use ast_ops::*;

use glob::glob;
use globwalk::glob as globwalk_glob;
use std::path::PathBuf;
use std::fs;

/// Resolve a path pattern that may be:
/// - A single file path
/// - A directory (searched recursively)
/// - A glob pattern
pub fn resolve_file_paths(path_pattern: &str) -> Result<Vec<PathBuf>, String> {
    let path = PathBuf::from(path_pattern);
    
    if path.is_file() {
        // Single file
        Ok(vec![path])
    } else if path.is_dir() {
        // Directory - search recursively
        let mut files = Vec::new();
        collect_files_recursive(&path, &mut files)?;
        Ok(files)
    } else if path_pattern.contains('*') || path_pattern.contains('?') || path_pattern.contains('[') || path_pattern.contains('{') {
        // Glob pattern - try globwalk first for brace expansion support
        if path_pattern.contains('{') {
            let mut paths = Vec::new();
            for entry in globwalk_glob(path_pattern).map_err(|e| format!("Invalid glob pattern: {}", e))? {
                let file_path = entry.map_err(|e| format!("Glob error: {}", e))?.into_path();
                if file_path.is_file() {
                    paths.push(file_path);
                }
            }
            Ok(paths)
        } else {
            // Use regular glob for simple patterns
            let mut paths = Vec::new();
            for entry in glob(path_pattern).map_err(|e| format!("Invalid glob pattern: {}", e))? {
                let file_path = entry.map_err(|e| format!("Glob error: {}", e))?;
                if file_path.is_file() {
                    paths.push(file_path);
                } else if file_path.is_dir() {
                    collect_files_recursive(&file_path, &mut paths)?;
                }
            }
            Ok(paths)
        }
    } else {
        // Path doesn't exist
        Err(format!("Path does not exist: {}", path_pattern))
    }
}

fn collect_files_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            collect_files_recursive(&path, files)?;
        }
    }
    
    Ok(())
}
