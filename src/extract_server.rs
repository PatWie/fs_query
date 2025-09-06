use rmcp::{
    Json, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};
use crate::server::*;

#[derive(Clone)]
pub struct ExtractSymbolsServer {
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for ExtractSymbolsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Extract symbols from source code files using tree-sitter parsing."
                    .to_string(),
            ),
        }
    }
}

#[tool_router]
impl ExtractSymbolsServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(name = "extract_symbols", description = "Parse source code files and extract symbols (functions, classes, structs, variables, etc.) with line ranges. Supports single files, directories (recursive), and advanced glob patterns with brace expansion. Examples: path_pattern='**/*.{h,hpp,cpp,cc}' (all C++ files), 'src/**/*.{rs,py}' (Rust/Python in src), '/path/to/project/' (entire directory), '**/*{Test,Spec}.js' (test files). Use filter to specify symbol type: 'function', 'class', 'struct', 'variable'. Returns symbols grouped by filename with precise line numbers for code navigation.")]
    pub async fn extract_symbols(&self, params: Parameters<ExtractSymbolsRequest>) -> Result<Json<Vec<FileSymbols>>, String> {
        extract_symbols(params.0).await.map(Json)
    }
}
