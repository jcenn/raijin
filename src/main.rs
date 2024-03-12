mod repo;
mod rjn_args;
use core::panic;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    u16, usize,
};

use crate::{
    repo::{add_entry, list_entries, remove_entry},
    rjn_args::{CommandType, RaijinArgs},
};

use clap::Parser;
use repo::{purge_repo, RjnEntry};

fn main() {
    let working_dir: PathBuf = env::current_dir().unwrap();
    let bin_dir: PathBuf = env::current_exe().unwrap();
    let args = RaijinArgs::parse();

    // println!("---DEBUG---");
    // println!("working directory location: {:?}", working_dir.display());
    // println!("bin location: {:?}", bin_dir.display());

    let mut db_path = bin_dir.clone();
    db_path.pop();
    db_path.push("db.yml");

    // println!("database location: {:?}", db_path.display());
    // println!("---DEBUG---");

    if args.alias.as_deref() == None && args.subcommand.is_none() {
        print_entries(list_entries(&db_path));
        return;
    }

    if args.alias.is_some() {
        let entries: Vec<RjnEntry> = list_entries(&db_path);
        let v = args.alias.as_deref().unwrap();
        let num = v.parse::<u16>();

        if let Ok(num) = num {
            if num < entries.len() as u16 {
                go_to_dir(&entries.get(num as usize).unwrap().dir)
            }
        } else {
            let matches: Vec<&RjnEntry> = entries
                .iter()
                .filter(|e| e.alias == v)
                .collect::<Vec<&RjnEntry>>();
            if matches.len() > 0 {
                go_to_dir(&matches.get(0).unwrap().dir);
            } else {
                panic!("No alias matches provided argument({})", v);
            }
        }
        return;
    }

    match &args.subcommand {
        Some(CommandType::Add(command)) => {
            if command.directory.to_str().unwrap() == ".".to_string() {
                let entry = RjnEntry {
                    alias: command.alias.clone(),
                    dir: working_dir.clone(),
                };
                match add_entry(&db_path, &entry) {
                    Ok(_) => {}
                    Err(_) => println!("ERR: entry with this alias already exists"),
                };
            }
            println!("added {} at {}", command.alias, command.directory.display());
        }
        Some(CommandType::Remove(command)) => {
            match remove_entry(&db_path, &command.alias) {
                Ok(_) => {},
                Err(_) => {},
            };
        }
        Some(CommandType::List) => {
            print_entries(list_entries(&db_path));
        }
        Some(CommandType::Purge) => {
            purge_repo(&db_path).unwrap()
        }
        Some(CommandType::Info) => {}
        None => todo!(),
    }

    // println!("hello {:?}", args);
}

fn print_entries(entries: Vec<RjnEntry>) {
    if entries.len() == 0{
        println!("No entries found, add one using [raijin add]");
        return;
    }

    // TODO: think of better padding than \t
    println!("id\talias\tdir");
    for i in 0..entries.len() {
        let e = &entries[i];
        println!("{}\t{}\t{}", i, e.alias, e.dir.to_str().unwrap());
    }
}

// Fun fact: It's not possible to change the working directory with rust, so I have to integrate
// some bash scripts
// Fun fact 2: source command doesn't work with my binaries for some reason
// This prints the path to the standard output which is then piped to a cd with a shell script
fn go_to_dir(p: &PathBuf) {
    // println!("going to dir: {}", p.to_str().unwrap());
    let _ = io::stdout().write_all(p.to_str().unwrap().as_bytes());
}
