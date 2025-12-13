use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Utc};
use std::env;
use rusqlite::Connection;
use anyhow::{Result, Context, bail};
use std::fs;

struct Project {
    name: String,
    description: String,
    path: PathBuf,
    date_created: DateTime<Utc>
}

impl Project {
    fn display(&self){
        println!("Project Name: {}\nLocation: {}\nCreated: {}\nDescription:\n{}\n", self.name, self.path.display(), self.date_created.format("%m/%d/%Y"), self.description);
    }
}

pub fn new_project(name: &str, init: bool) -> Result<()> {
    let date = Utc::now();
    let path: PathBuf = env::current_dir().expect("Failed to get current dir");
    let name_str = name.to_string();
    let description = "".to_string();
    let path_str = path.display().to_string();

    let root = match env::var("QCLI_ENV") {
        Ok(val) => val,
        Err(_) => bail!("QCLI env variable is not set"),
    };

    let mut db_path = PathBuf::from(root);
    db_path.push("forge.db");
    let connection = Connection::open(db_path)?;
    
    connection.execute("INSERT INTO projects (name, description, path, date_created) VALUES (?1, ?2, ?3, ?4)",
    (name_str, description, path_str, date),)?;
    
    Ok(())
}

pub fn init(path: &Path) -> Result<()> {
    // create initialization directory if needed
    if !path.exists() {
        println!("Creating path: {}", path.display());
        fs::create_dir_all(path).expect("Failed to create directory");
    }

    // Need to set env variable to designate the working directory.
    println!("Set QCLI_ENV environment variable to {}", path.display());

    // setup sqlite tables
    let mut db_path = path.to_path_buf();
    db_path.push("forge.db");
    let connection = Connection::open(db_path.as_path()).expect("Failed to load database");
    let res = connection.execute("CREATE TABLE IF NOT EXISTS projects
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        description TEXT,
                        path TEXT,
                        date_create TEXT", (),)?;

    connection.execute("CREATE TABLE IF NOT EXISTS project_log
                        id INTEGER PRIMARY KEY,
                        project_id INTEGER,
                        log TEXT,
                        date TEXT", (),)?;
    
    Ok(())

}