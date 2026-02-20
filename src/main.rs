mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;

use std::process;

use clap::Parser;

use cli::{AuthAction, Cli, CollectionAction, Command, DocumentAction};
use client::OutlineClient;
use error::AppError;
use output::print_error;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        print_error(&e);
        process::exit(e.exit_code());
    }
}

async fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Auth { action, token, url } => match action {
            Some(AuthAction::Status) => commands::auth::status().await,
            None => commands::auth::run(token, url).await,
        },

        Command::Documents { action } => {
            let cfg = config::load()?;
            let client = OutlineClient::from_config(&cfg);

            match action {
                DocumentAction::List {
                    collection_id,
                    offset,
                    limit,
                } => commands::documents::list(&client, collection_id, offset, limit).await,

                DocumentAction::Get { id } => commands::documents::get(&client, &id).await,

                DocumentAction::Create {
                    title,
                    collection_id,
                    text,
                } => commands::documents::create(&client, &title, &collection_id, text).await,

                DocumentAction::Update { id, title, text } => {
                    commands::documents::update(&client, &id, title, text).await
                }

                DocumentAction::Delete { id, permanent } => {
                    commands::documents::delete(&client, &id, permanent).await
                }

                DocumentAction::Search {
                    query,
                    collection_id,
                    limit,
                } => commands::documents::search(&client, &query, collection_id, limit).await,

                DocumentAction::Export { id } => commands::documents::export(&client, &id).await,
            }
        }

        Command::Collections { action } => {
            let cfg = config::load()?;
            let client = OutlineClient::from_config(&cfg);

            match action {
                CollectionAction::List { offset, limit } => {
                    commands::collections::list(&client, offset, limit).await
                }
                CollectionAction::Get { id } => commands::collections::get(&client, &id).await,
                CollectionAction::Documents { id } => {
                    commands::collections::documents(&client, &id).await
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_help() {
        // --help exits with error code by design
        let result = Cli::try_parse_from(["outline", "--help"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parse_auth() {
        let cli =
            Cli::try_parse_from(["outline", "auth", "--token", "tok", "--url", "https://x.com"])
                .unwrap();
        match cli.command {
            Command::Auth { token, url, .. } => {
                assert_eq!(token.unwrap(), "tok");
                assert_eq!(url.unwrap(), "https://x.com");
            }
            _ => panic!("expected Auth command"),
        }
    }

    #[test]
    fn test_cli_parse_documents_list() {
        let cli = Cli::try_parse_from(["outline", "documents", "list", "--limit", "10"]).unwrap();
        match cli.command {
            Command::Documents {
                action: DocumentAction::List { limit, .. },
            } => {
                assert_eq!(limit, Some(10));
            }
            _ => panic!("expected Documents List command"),
        }
    }

    #[test]
    fn test_cli_parse_documents_search() {
        let cli = Cli::try_parse_from(["outline", "documents", "search", "test query"]).unwrap();
        match cli.command {
            Command::Documents {
                action: DocumentAction::Search { query, .. },
            } => {
                assert_eq!(query, "test query");
            }
            _ => panic!("expected Documents Search command"),
        }
    }

    #[test]
    fn test_cli_parse_collections_get() {
        let cli = Cli::try_parse_from(["outline", "collections", "get", "col-123"]).unwrap();
        match cli.command {
            Command::Collections {
                action: CollectionAction::Get { id },
            } => {
                assert_eq!(id, "col-123");
            }
            _ => panic!("expected Collections Get command"),
        }
    }
}
