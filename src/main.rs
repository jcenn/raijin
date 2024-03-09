use std::{env::{self, args}, fmt::Debug, fs, path::PathBuf};

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct RaijinArgs {
    /// Alias of the directory to navigate to
    alias: Option<String>,

    #[clap(subcommand)]
    subcommand: Option<CommandType>,
}

#[derive(Debug, Subcommand)]
enum CommandType {
    /// Add a directory
    Add(AddCommand),

    /// Remove a directory
    Remove(RemoveCommand),

    /// List all entries
    List,
}

#[derive(Debug, Args)]
struct AddCommand {
    /// Alias of new entry
    alias: String,

    /// Directory of new entry
    directory: PathBuf,
}

#[derive(Debug, Args)]
struct RemoveCommand {
    /// Alias of the entry you wish to remove
    alias: String,
}

fn main() {
    let path = env::current_dir().unwrap();
    let args = RaijinArgs::parse();

    println!("location arg: {:?}", path.display());
    // TODO: check if yaml exists, if not create a new one (ask user for permision?)

    if args.alias.as_deref() == None && args.subcommand.is_none() {
        println!("no alias provided, listing all entries");
        print_entries(list_registered_entries());
        return;
    }
    match &args.subcommand {
        Some(CommandType::Add(command)) => {
            println!("adding {:?} at {:?}", command.alias, command.directory)
        }
        Some(CommandType::Remove(..)) => {
            println!("adding")
        }
        Some(CommandType::List) => {
            println!("adding")
        }
        None => todo!(),
    }

    println!("hello {:?}", args);
}

struct RjnEntry {
    alias: String,
    dir: PathBuf,
}

// Should I use Arc<T> instead?
fn list_registered_entries() -> Vec<RjnEntry> {
    vec![]
}

fn print_entries(entries: Vec<RjnEntry>) {
    println!("entries: ");
}
