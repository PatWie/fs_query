use anyhow::Result;
use clap::Parser as ClapParser;
use rmcp::{
    ServiceExt,
    transport::stdio,
};
use tracing_subscriber;

mod ast;
mod server;
mod extract_server;

use server::*;
use extract_server::ExtractSymbolsServer;

#[derive(ClapParser, Debug)]
#[command(name = "fs_query")]
#[command(about = "File MCP Server")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    ExtractSymbols {
        #[arg(short, long)]
        file_path: String,
        #[arg(short = 's', long)]
        symbols: Option<String>,
        #[arg(long)]
        name_regex: Option<String>,
        #[arg(short, long)]
        pretty: bool,
    },
    Mcp,
}



#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let args = Args::parse();

    match args.command {
        Some(Commands::ExtractSymbols { file_path, symbols, name_regex, pretty }) => {
            let filter_kind = symbols.as_deref().and_then(|f| match f {
                "function" => Some(ast::SymbolKind::Function),
                "class" => Some(ast::SymbolKind::Class),
                "struct" => Some(ast::SymbolKind::Struct),
                "variable" => Some(ast::SymbolKind::Variable),
                "method" => Some(ast::SymbolKind::Method),
                "enum" => Some(ast::SymbolKind::Enum),
                "trait" => Some(ast::SymbolKind::Trait),
                "interface" => Some(ast::SymbolKind::Interface),
                "type" => Some(ast::SymbolKind::Type),
                _ => None,
            });
            let req = ExtractSymbolsRequest { 
                path_pattern: file_path, 
                filter: filter_kind, 
                start_line: None, 
                end_line: None 
            };
            match extract_symbols(req).await {
                Ok(mut result) => {
                    if let Some(pattern) = name_regex {
                        let re = match regex::Regex::new(&pattern) {
                            Ok(re) => re,
                            Err(e) => {
                                eprintln!("Invalid regex pattern: {}", e);
                                std::process::exit(1);
                            }
                        };
                        // Filter symbols within each file
                        for file_symbols in &mut result {
                            file_symbols.symbols.retain(|symbol| re.is_match(&symbol.name));
                        }
                        // Remove files with no matching symbols
                        result.retain(|file_symbols| !file_symbols.symbols.is_empty());
                    }
                    if pretty {
                        for file_symbols in &result {
                            println!("{}", file_symbols.filename);
                            for symbol in &file_symbols.symbols {
                                let kind_name = match symbol.kind {
                                    ast::SymbolKind::Function => "[FUNCTION]",
                                    ast::SymbolKind::Class => "[CLASS]",
                                    ast::SymbolKind::Struct => "[STRUCT]",
                                    ast::SymbolKind::Variable => "[VARIABLE]",
                                    ast::SymbolKind::Method => "[METHOD]",
                                    ast::SymbolKind::Enum => "[ENUM]",
                                    ast::SymbolKind::Trait => "[TRAIT]",
                                    ast::SymbolKind::Interface => "[INTERFACE]",
                                    ast::SymbolKind::Type => "[TYPE]",
                                };
                                println!("  {} {} (lines {}-{})", kind_name, symbol.name, symbol.start_line, symbol.end_line);
                            }
                            println!();
                        }
                    } else {
                        println!("{:?}", result);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(Commands::Mcp) => {
            tracing::info!("Starting MCP server");
            let server = ExtractSymbolsServer::new();
            let service = server.serve(stdio()).await.inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;
            service.waiting().await?;
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}
