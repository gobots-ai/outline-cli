use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "outline", about = "CLI for the Outline Wiki API", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authenticate with an Outline instance
    Auth {
        #[command(subcommand)]
        action: Option<AuthAction>,

        /// API token (non-interactive)
        #[arg(long)]
        token: Option<String>,

        /// Base URL of your Outline instance (non-interactive)
        #[arg(long)]
        url: Option<String>,
    },

    /// Manage documents
    Documents {
        #[command(subcommand)]
        action: DocumentAction,
    },

    /// Manage collections
    Collections {
        #[command(subcommand)]
        action: CollectionAction,
    },
}

#[derive(Subcommand)]
pub enum AuthAction {
    /// Check authentication status
    Status,
}

#[derive(Subcommand)]
pub enum DocumentAction {
    /// List documents
    List {
        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Pagination offset
        #[arg(long)]
        offset: Option<u32>,

        /// Number of results to return
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Get a document by ID
    Get {
        /// Document ID
        id: String,
    },

    /// Create a new document
    Create {
        /// Document title
        #[arg(long)]
        title: String,

        /// Collection to create the document in
        #[arg(long)]
        collection_id: String,

        /// Markdown body text
        #[arg(long)]
        text: Option<String>,
    },

    /// Update an existing document
    Update {
        /// Document ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New markdown body text
        #[arg(long)]
        text: Option<String>,
    },

    /// Delete a document
    Delete {
        /// Document ID
        id: String,

        /// Permanently delete instead of archiving
        #[arg(long)]
        permanent: bool,
    },

    /// Search documents
    Search {
        /// Search query
        query: String,

        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Number of results to return
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Export a document as markdown
    Export {
        /// Document ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum CollectionAction {
    /// List all collections
    List {
        /// Pagination offset
        #[arg(long)]
        offset: Option<u32>,

        /// Number of results to return
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Get a collection by ID
    Get {
        /// Collection ID
        id: String,
    },

    /// List documents in a collection as a tree
    Documents {
        /// Collection ID
        id: String,
    },
}
