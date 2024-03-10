use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RjnEntry {
    pub alias: String,
    pub dir: PathBuf,
}

// Should I use Arc<T> instead?
pub fn list_entries(db_path: &PathBuf) -> Vec<RjnEntry> {
    // I'm not 100% sure why this need write access, but it does
    // Update: I do, create needs write access, it's in the docs
    let db = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(db_path);
    let mut entries: Vec<RjnEntry> = vec![];
    // println!("DEBUG: db path: {}", db_path.display());
    match db {
        Ok(_) => {
            let mut content = String::new();
            db.unwrap().read_to_string(&mut content).unwrap();
            // println!("content: {}", &content);
            entries = serde_yaml::from_str(&content).unwrap();
        }
        Err(err) => {
            println!("ERR: {:?}", err.to_string())
        }
    };
    return entries;
}

pub fn add_entry(db_path: &PathBuf, entry: &RjnEntry) -> Result<(), ()> {
    println!("db path: {}", db_path.display());
    //TODO: check if entry with that alias doesn't already exist
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

    return Ok(());
}

pub fn remove_entry(db_path: &PathBuf, alias: &str) -> Result<(), ()> {
    Ok(())
}

pub fn purge_repo(db_path: &PathBuf) -> Result<(), ()> {
    Ok(())
}
