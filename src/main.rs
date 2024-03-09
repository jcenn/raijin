use std::{
    env,
    fmt::Debug,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

fn main() {
    let working_dir: PathBuf = env::current_dir().unwrap();
    let bin_dir: PathBuf = PathBuf::from_str(&env::args().collect::<Vec<String>>()[0]).unwrap();
    let args = RaijinArgs::parse();

    println!("---DEBUG---");
    println!("working directory location: {:?}", working_dir.display());
    println!("bin location: {:?}", bin_dir.display());

    let mut db_path = bin_dir.clone();
    db_path.pop();
    db_path.push("db.yml");

    println!("database location: {:?}", db_path.display());
    println!("---DEBUG---");

    if args.alias.as_deref() == None && args.subcommand.is_none() {
        print_entries(list_entries(&db_path));
        return;
    }

    if args.alias.is_some() {
        let v = args.alias.as_deref().unwrap();
        // if alias is a number and less than entries.len(), go to entry by index
        // else go to index by name
        // else panic
        println!("going to alias: {}", v);
        return;
    }

    match &args.subcommand {
        Some(CommandType::Add(command)) => {
            if command.directory.to_str().unwrap() == ".".to_string() {
                let entry = RjnEntry {
                    alias: command.alias.clone(),
                    dir: working_dir.clone(),
                };
                add_entry(&db_path, &entry);
            }
            println!("adding {:?} at {:?}", command.alias, command.directory)
        }
        Some(CommandType::Remove(..)) => {
            println!("adding")
        }
        Some(CommandType::List) => {
            print_entries(list_entries(&db_path));
        }
        Some(CommandType::Purge) => {}
        None => todo!(),
    }

    println!("hello {:?}", args);
}

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

    /// Removes all entries
    Purge,
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

#[derive(Serialize, Deserialize, Debug)]
struct RjnEntry {
    alias: String,
    dir: PathBuf,
}

// Should I use Arc<T> instead?
fn list_entries(db_path: &PathBuf) -> Vec<RjnEntry> {
    let mut db = OpenOptions::new()
        .create(true)
        .read(true)
        .open(db_path)
        .unwrap();
    let mut content = String::new();
    db.read_to_string(&mut content).unwrap();
    println!("content: {}", &content);
    let entries: Vec<RjnEntry> = serde_yaml::from_str(&content).unwrap();
    return entries;
}

fn add_entry(db_path: &PathBuf, entry: &RjnEntry) {
    let mut db = OpenOptions::new()
        .create(true)
        .read(true)
        .open(db_path)
        .unwrap();
    let mut entries = list_entries(db_path);
    entries.push(RjnEntry {
        alias: entry.alias.clone(),
        dir: entry.dir.clone(),
    });

    let mut db = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(db_path)
        .unwrap();
    db.write_all(serde_yaml::to_string(&entries).unwrap().as_bytes())
        .unwrap();
}

fn print_entries(entries: Vec<RjnEntry>) {
    println!("id\talias\tdir");
    for i in 0..entries.len() {
        let e = &entries[i];
        println!("{}\t{}\t{}", i, e.alias, e.dir.to_str().unwrap());
    }
}
