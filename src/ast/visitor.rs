use super::symbol::Symbol;
use tree_sitter::Node;

pub trait LanguageVisitor {
    fn visit(&mut self, node: &Node, source_code: &str);
    fn get_symbols(self) -> Vec<Symbol>;
}
