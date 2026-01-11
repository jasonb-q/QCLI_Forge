use std::path::PathBuf;
use chrono::{DateTime, Utc};
use std::env;
use rusqlite::{Connection, Row};
use anyhow::{Result, bail};
use std::error::Error;
use std::fs::File;
use crate::utils;
use serde_yaml;

struct Config {
    type: String,
    sub_folders: String
}

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

fn convert_project_row(row: &Row) -> rusqlite::Result<Project> {
    let path_str: String = row.get(3)?;
    let date_str: String = row.get(4)?;
    let path: PathBuf = PathBuf::from(path_str);
    let date_created: DateTime<Utc> = date_str.parse().unwrap();
    Ok(Project {
        name: row.get(1)?,
        description: row.get(2)?,
        path: path,
        date_created: date_created
    })
}

pub fn list_project(name: &str) -> Result<()> {
    let db_path: PathBuf = utils::get_db_path()?;
    let conn = Connection::open(db_path)?;

    let mut query;
    if name == "!!" {
        query = conn.prepare("SELECT * FROM projects")?;
    } else {
        query = conn.prepare("SELECT * FROM projects WHERE name = ?1")?;
    }
        
    let project_iter = if name == "!!" {
        query.query_map([], convert_project_row)?
    } else {
        query.query_map([name], convert_project_row)?
    };

    for proj in project_iter {
        proj.unwrap().display();
    }

    Ok(())
}

pub fn load_config_yaml(config: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(config)?;
    let conf = serde_yaml::from_reader(file)?;
    println!("yaml file: {}", conf);
    Ok(())
}

pub fn new_project(name: &str, proj_desc: &str, _init: bool, config: &str) -> Result<()> {
    let date = Utc::now();
    let name_str = name.to_string();
    let description = proj_desc.to_string();
    
    // Get current directory (Project dir)
    let path: PathBuf = env::current_dir().expect("Failed to get current dir");
    let path_into = path.into_os_string().into_string();
    let path_str = match path_into {
        Ok(val) => val,
        Err(_) => {
            bail!("Couldn't convert path into string");
        }
    };

    let db_path: PathBuf = utils::get_db_path()?;
    let connection = Connection::open(db_path)?;

    if config.len() > 0 {
        list_project(config);
    }

    //connection.execute("INSERT INTO projects (name, description, path, date_created) VALUES (?1, ?2, ?3, ?4)",
    //(name_str, description, path_str, date),)?;
    
    Ok(())
}
