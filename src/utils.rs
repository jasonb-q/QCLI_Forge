use std::path::{Path, PathBuf};
use chrono::Local;
use std::env;
use rusqlite::{Connection,Result,params};
use std::fs;

struct Project {
    name: String,
    description: String,
    path: PathBuf,
    date_created: String
}

impl Project {
    fn display(&self){
        println!("Project Name: {}\nLocation: {}\nCreated: {}\nDescription:\n{}\n", self.name, self.path.display(), self.date_created, self.description);
    }
}

pub fn new_project(name: &str, init: bool) {
    let date = Local::now();
    let path: PathBuf = env::current_dir().expect("Failed to get current dir");
    let new_project = Project {
        name: name.to_string(),
        description: "".to_string(),
        path: path,
        date_created: format!("{}", date.format("%m/%d/%Y"))
    };

    new_project.display();
}

pub fn init(path: &Path) {
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
    connection.execute("CREATE TABLE IF NOT EXISTS projects
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        description TEXT,
                        path TEXT,
                        date_create DATE", (),);
    
    connection.execute("CREATE TABLE IF NOT EXISTS project_log
                        id INTEGER PRIMARY KEY,
                        project_id INTEGER,
                        log TEXT,
                        date DATE", (),);

}