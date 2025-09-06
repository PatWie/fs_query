use super::symbol::{Symbol, SymbolKind};
use super::visitor::LanguageVisitor;
use tree_sitter::Node;

pub struct CppVisitor {
    symbols: Vec<Symbol>,
}

impl CppVisitor {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    fn extract_function(&self, node: &Node, source: &str) -> Option<Symbol> {
        let declarator = node.child_by_field_name("declarator")?;
        let func_declarator = declarator.child_by_field_name("declarator")
            .unwrap_or(declarator);
        
        let name = func_declarator.utf8_text(source.as_bytes()).ok()?.to_string();
        
        Some(Symbol {
            kind: SymbolKind::Function,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(func_declarator.byte_range()),
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

    fn extract_struct(&self, node: &Node, source: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = name_node.utf8_text(source.as_bytes()).ok()?.to_string();
        
        Some(Symbol {
            kind: SymbolKind::Struct,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(name_node.byte_range()),
            body_range: node.child_by_field_name("body").map(|b| b.byte_range()),
        })
    }

    fn extract_variable(&self, node: &Node, source: &str) -> Option<Symbol> {
        let declarator = node.child_by_field_name("declarator")?;
        
        // Try to find the identifier within the declarator
        let name = if let Some(identifier) = declarator.child_by_field_name("declarator") {
            // Handle cases like "auto variable_name = ..."
            identifier.utf8_text(source.as_bytes()).ok()?.to_string()
        } else if declarator.kind() == "identifier" {
            // Direct identifier
            declarator.utf8_text(source.as_bytes()).ok()?.to_string()
        } else {
            // Fallback: look for first identifier child
            let mut cursor = declarator.walk();
            cursor.goto_first_child();
            loop {
                let current = cursor.node();
                if current.kind() == "identifier" {
                    return Some(Symbol {
                        kind: SymbolKind::Variable,
                        name: current.utf8_text(source.as_bytes()).ok()?.to_string(),
                        start_line: node.start_position().row + 1,
                        end_line: node.end_position().row + 1,
                        full_range: node.byte_range(),
                        name_range: Some(current.byte_range()),
                        body_range: None,
                    });
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            declarator.utf8_text(source.as_bytes()).ok()?.to_string()
        };
        
        Some(Symbol {
            kind: SymbolKind::Variable,
            name,
            start_line: node.start_position().row + 1,
            end_line: node.end_position().row + 1,
            full_range: node.byte_range(),
            name_range: Some(declarator.byte_range()),
            body_range: None,
        })
    }
}

impl LanguageVisitor for CppVisitor {
    fn visit(&mut self, node: &Node, source_code: &str) {
        let maybe_symbol = match node.kind() {
            "function_definition" => self.extract_function(node, source_code),
            "class_specifier" => self.extract_class(node, source_code),
            "struct_specifier" => self.extract_struct(node, source_code),
            "declaration" => self.extract_variable(node, source_code),
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
