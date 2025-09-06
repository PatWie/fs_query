pub mod symbol;
pub mod visitor;
pub mod parser;
pub mod cpp_visitor;
pub mod python_visitor;
pub mod js_visitor;
pub mod go_visitor;

#[cfg(test)]
mod tests_cpp;
#[cfg(test)]
mod tests_python;
#[cfg(test)]
mod tests_js;
#[cfg(test)]
mod tests_go;

pub use symbol::SymbolKind;
pub use parser::{CodeParser, get_language};
