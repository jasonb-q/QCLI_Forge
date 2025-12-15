use std::path::{Path, PathBuf};
use std::env;
use rusqlite::Connection;
use anyhow::Result;
use std::fs;

pub fn get_db_path() -> Result<PathBuf> {
    let root = env::var("QCLI_ENV").map_err(|_| anyhow::anyhow!("QCLI env variable isn't set!"))?; 
    let mut db_path = PathBuf::from(root);
    db_path.push("forge.db");
    Ok(db_path)
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
    connection.execute("CREATE TABLE IF NOT EXISTS projects (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        description TEXT,
                        path TEXT,
                        date_created TEXT)", (),)?;

    connection.execute("CREATE TABLE IF NOT EXISTS project_log (
                        id INTEGER PRIMARY KEY,
                        project_id INTEGER,
                        log TEXT,
                        date TEXT)", (),)?;
    
    Ok(())

}