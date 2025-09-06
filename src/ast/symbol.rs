#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Function,
    Class,
    Struct,
    Variable,
    Method,
    Enum,
    Trait,
    Interface,
    Type,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub full_range: std::ops::Range<usize>,
    pub name_range: Option<std::ops::Range<usize>>,
    pub body_range: Option<std::ops::Range<usize>>,
}
