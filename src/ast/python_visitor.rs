use super::symbol::{Symbol, SymbolKind};
use super::visitor::LanguageVisitor;
use tree_sitter::Node;

pub struct PythonVisitor {
    symbols: Vec<Symbol>,
}

impl PythonVisitor {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    fn extract_function(&self, node: &Node, source: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = name_node.utf8_text(source.as_bytes()).ok()?.to_string();
        
        Some(Symbol {
            kind: SymbolKind::Function,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(name_node.byte_range()),
            body_range: node.child_by_field_name("body").map(|n| n.byte_range()),
        })
    }

    fn extract_class(&self, node: &Node, source: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = name_node.utf8_text(source.as_bytes()).ok()?.to_string();
        
        Some(Symbol {
            kind: SymbolKind::Class,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(name_node.byte_range()),
            body_range: node.child_by_field_name("body").map(|n| n.byte_range()),
        })
    }
}

impl LanguageVisitor for PythonVisitor {
    fn visit(&mut self, node: &Node, source_code: &str) {
        let maybe_symbol = match node.kind() {
            "function_definition" => self.extract_function(node, source_code),
            "class_definition" => self.extract_class(node, source_code),
            _ => None,
        };

        if let Some(symbol) = maybe_symbol {
            self.symbols.push(symbol);
        }

        // Recurse into children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit(&child, source_code);
            }
        }
    }

    fn get_symbols(self) -> Vec<Symbol> {
        self.symbols
    }
}
