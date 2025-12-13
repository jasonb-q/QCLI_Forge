use std::path::PathBuf;
use structopt::StructOpt;
use rusqlite::Connection;
use std::env;
use anyhow::Result;

mod utils;

#[derive(StructOpt, Debug)]
struct Forge {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Init {
        #[structopt(short)]
        path: Option<PathBuf>,
    },
    project(Project),
}

#[derive(StructOpt, Debug)]
enum Project {
    new {
        #[structopt(short)]
        name: String,
        #[structopt(short)]
        init: bool,
    },
}


fn main() {
    let opt = Forge::from_args();
    match &opt.command {
        Command::project(Project::new {name, init}) => {
            utils::new_project(name, *init);
        }
        Command::Init {path} => {
            match path {
                Some(p) => {
                    let result = utils::init(p.as_path());
                    match result {
                        Ok(val) => {
                            println!("success!");
                        },
                        Err(e) => {
                            println!("Error {}", e);
                        }
                    }
                }
                None => {
                    let home_dir: Option<PathBuf> = env::home_dir();
                    match home_dir {
                        Some(mut home) => {
                            home.push("forge");
                            utils::init(home.as_path());
                        }
                        None => {
                            println!("Failed to initialize qcli.\nNo home directory found. Supply an initialization dir.");
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", opt.command);
}
