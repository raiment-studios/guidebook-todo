use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A terminal-based TODO application")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize guidebook-todo for first-time use
    Init,

    /// Add a new TODO
    Add {
        /// TODO title
        title: Option<String>,
        #[arg(long)]
        /// Create TODO with defaults, bypass TUI
        quick: bool,
    },

    /// List TODOs
    List {
        #[arg(long)]
        /// Filter by status
        status: Option<String>,
        #[arg(long)]
        /// Filter by category
        category: Option<String>,
        #[arg(long)]
        /// Filter by priority
        priority: Option<String>,
        #[arg(long)]
        /// Filter by tags
        tags: Option<String>,
        #[arg(long)]
        /// Show all TODOs regardless of status
        all: bool,
    },

    /// Update a TODO
    Update {
        /// TODO ID
        id: u32,
        #[arg(long)]
        /// New status
        status: Option<String>,
        #[arg(long)]
        /// New priority
        priority: Option<String>,
        #[arg(long)]
        /// Add/remove tags (+tag,-tag)
        tags: Option<String>,
        #[arg(long)]
        /// New category
        category: Option<String>,
        #[arg(long)]
        /// New project
        project: Option<String>,
        #[arg(long)]
        /// Update notes
        notes: Option<String>,
    },

    /// Delete TODOs
    Delete {
        /// TODO ID
        id: Option<u32>,
        #[arg(long)]
        /// Delete all in category
        category: Option<String>,
        #[arg(long)]
        /// Delete all with status
        status: Option<String>,
    },

    /// Search TODOs interactively
    Search {
        /// Pre-fill search term
        query: Option<String>,
    },

    /// Edit a TODO interactively
    Edit {
        /// TODO ID
        id: u32,
    },

    /// Show TODO details
    Show {
        /// TODO ID
        id: u32,
    },

    /// Show statistics
    Stats,

    /// Push changes to GitHub repository
    Push {
        #[arg(long)]
        /// Commit message for the push
        message: Option<String>,
        #[arg(long)]
        /// Force push even if no changes detected
        force: bool,
    },

    /// Open the TODO file in VS Code
    Code,
}
