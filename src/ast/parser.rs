use super::symbol::{Symbol, SymbolKind};
use super::visitor::{LanguageVisitor};
use super::cpp_visitor::CppVisitor;
use super::python_visitor::PythonVisitor;
use super::js_visitor::JsVisitor;
use super::go_visitor::GoVisitor;
use std::collections::HashSet;
use tree_sitter::{Language, Parser};

pub struct CodeParser {
    parser: Parser,
}

impl CodeParser {
    pub fn new(language: Language) -> Result<Self, String> {
        let mut parser = Parser::new();
        parser.set_language(&language)
            .map_err(|e| format!("Failed to set language: {}", e))?;
        
        Ok(Self { parser })
    }

    pub fn parse_with_visitor<V: LanguageVisitor>(
        &mut self,
        source_code: &str,
        mut visitor: V,
    ) -> Result<Vec<Symbol>, String> {
        let tree = self.parser.parse(source_code, None)
            .ok_or("Failed to parse source code")?;
        
        visitor.visit(&tree.root_node(), source_code);
        Ok(visitor.get_symbols())
    }

    pub fn extract_symbols(
        &mut self,
        source_code: &str,
        file_path: &str,
        filter: Option<HashSet<SymbolKind>>,
    ) -> Result<Vec<Symbol>, String> {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let mut symbols = match ext {
            "cpp" | "cc" | "cxx" | "c" | "h" | "hpp" => {
                let visitor = CppVisitor::new();
                self.parse_with_visitor(source_code, visitor)?
            }
            "py" => {
                let visitor = PythonVisitor::new();
                self.parse_with_visitor(source_code, visitor)?
            }
            "js" | "ts" => {
                let visitor = JsVisitor::new();
                self.parse_with_visitor(source_code, visitor)?
            }
            "go" => {
                let visitor = GoVisitor::new();
                self.parse_with_visitor(source_code, visitor)?
            }
            _ => {
                let visitor = PythonVisitor::new(); // fallback
                self.parse_with_visitor(source_code, visitor)?
            }
        };

        // Apply filtering
        if let Some(filter) = filter {
            symbols.retain(|s| filter.contains(&s.kind));
        }

        Ok(symbols)
    }
}

pub fn get_language(file_path: &str) -> Option<Language> {
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match ext {
        "cpp" | "cc" | "cxx" | "c" | "h" | "hpp" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "py" => Some(tree_sitter_python::LANGUAGE.into()),
        "rs" => Some(tree_sitter_rust::LANGUAGE.into()),
        "js" | "ts" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "go" => Some(tree_sitter_go::LANGUAGE.into()),
        _ => None,
    }
}
