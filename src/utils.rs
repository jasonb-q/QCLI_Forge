use std::path::{Path, PathBuf};
use chrono::Local;
use std::env;
use rusqlite::{Connection,Result,params};

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