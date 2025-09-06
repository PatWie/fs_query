use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::ast;
use std::fs;
use std::collections::HashMap;

// Request structs
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExtractSymbolsRequest {
    pub path_pattern: String,
    pub filter: Option<ast::SymbolKind>,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
}

// Response structs
#[derive(Debug, Serialize, JsonSchema)]
pub struct Symbol {
    pub name: String,
    pub kind: ast::SymbolKind,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct FileSymbols {
    pub filename: String,
    pub symbols: Vec<Symbol>,
}

// Handler functions
pub async fn extract_symbols(params: ExtractSymbolsRequest) -> Result<Vec<FileSymbols>, String> {
    let mut file_symbols_map: HashMap<String, Vec<Symbol>> = HashMap::new();
    let file_paths = crate::server::resolve_file_paths(&params.path_pattern)?;
    
    for file_path in file_paths {
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Only process files that have supported language extensions
        if let Some(language) = ast::get_language(&file_path_str) {
            if let Ok(content) = fs::read_to_string(&file_path) {
                if let Ok(mut parser) = ast::CodeParser::new(language) {
                    let filter = params.filter.map(|kind| {
                        let mut set = std::collections::HashSet::new();
                        set.insert(kind);
                        set
                    });

                    if let Ok(ast_symbols) = parser.extract_symbols(&content, &file_path_str, filter) {
                        let symbols: Vec<Symbol> = ast_symbols.into_iter()
                            .filter(|s| {
                                let in_range = match (params.start_line, params.end_line) {
                                    (Some(start), Some(end)) => s.start_line >= start && s.start_line <= end,
                                    (Some(start), None) => s.start_line >= start,
                                    (None, Some(end)) => s.start_line <= end,
                                    (None, None) => true,
                                };
                                in_range
                            })
                            .map(|s| Symbol {
                                name: s.name,
                                kind: s.kind,
                                start_line: s.start_line,
                                end_line: s.end_line,
                            })
                            .collect();
                        
                        if !symbols.is_empty() {
                            file_symbols_map.insert(file_path_str, symbols);
                        }
                    }
                }
            }
        }
    }

    let result: Vec<FileSymbols> = file_symbols_map.into_iter()
        .map(|(filename, symbols)| FileSymbols { filename, symbols })
        .collect();

    Ok(result)
}
