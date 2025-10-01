use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "Simple todo CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    Add {
        title: String,
        #[arg(short, long)]
        details: Option<String>,
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        completed: bool,
    },
    Done {
        id: String,
    },
    Remove {
        id: String,
    },
    Edit {
        id: String,
        title: Option<String>,
        details: Option<String>,
    },
}
