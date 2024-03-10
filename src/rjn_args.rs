use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct RaijinArgs {
    /// Alias of the directory to navigate to
    pub alias: Option<String>,

    #[clap(subcommand)]
    pub subcommand: Option<CommandType>,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Add a directory
    Add(AddCommand),

    /// Remove a directory
    Remove(RemoveCommand),

    /// List all entries
    List,

    /// Removes all entries
    Purge,
    
    /// Displays information about the tool
    Info,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// Alias of new entry
    pub alias: String,

    /// Directory of new entry
    pub directory: PathBuf,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// Alias of the entry you wish to remove
    pub alias: String,
}
