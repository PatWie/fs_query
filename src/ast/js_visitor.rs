use super::symbol::{Symbol, SymbolKind};
use super::visitor::LanguageVisitor;
use tree_sitter::Node;

pub struct JsVisitor {
    symbols: Vec<Symbol>,
}

impl JsVisitor {
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

    fn extract_variable(&self, node: &Node, source: &str) -> Option<Symbol> {
        // For variable_declarator
        let name_node = node.child_by_field_name("name")?;
        let name = name_node.utf8_text(source.as_bytes()).ok()?.to_string();
        
        Some(Symbol {
            kind: SymbolKind::Variable,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(name_node.byte_range()),
            body_range: node.child_by_field_name("value").map(|n| n.byte_range()),
        })
    }
}

impl LanguageVisitor for JsVisitor {
    fn visit(&mut self, node: &Node, source_code: &str) {
        let maybe_symbol = match node.kind() {
            "function_declaration" | "function" => self.extract_function(node, source_code),
            "class_declaration" => self.extract_class(node, source_code),
            "variable_declarator" => self.extract_variable(node, source_code),
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
