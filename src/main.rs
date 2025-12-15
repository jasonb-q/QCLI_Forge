use std::path::PathBuf;
use structopt::StructOpt;
use std::env;

mod utils;
mod project_functions;

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
    Project(Project),
}

#[derive(StructOpt, Debug)]
enum Project {
    New {
        #[structopt(short)]
        name: String,
        #[structopt(short)]
        description: Option<String>,
        #[structopt(short)]
        init: bool,
    },
    List {
        #[structopt(short)]
        name: Option<String>,
    },
}


fn main() {
    let opt = Forge::from_args();
    match &opt.command {
        Command::Project(Project::New {name, description, init}) => {
            let proj_desc = match description {
                Some(desc) => desc,
                None => ""
            };
            let result = project_functions::new_project(name, proj_desc, *init);
            match result {
                Ok(_) => println!("success!"),
                Err(e) => println!("Error: {}", e)
            };
        }
        Command::Project(Project::List {name}) => {
            let name_str = match name {
                Some(n) => n,
                None => "!!"
            };
            let result = project_functions::list_project(name_str);
            match result {
                Ok(_) => println!("success!"),
                Err(e) => println!("Error: {e}")
            };
        }
        Command::Init {path} => {
            match path {
                Some(p) => {
                    let result = utils::init(p.as_path());
                    match result {
                        Ok(_) => {
                            println!("success!");
                        },
                        Err(e) => {
                            println!("Error {}", e);
                        }
                    };
                }
                None => {
                    let home_dir: Option<PathBuf> = env::home_dir();
                    match home_dir {
                        Some(mut home) => {
                            home.push("forge");
                            let result = utils::init(home.as_path());
                            match result {
                                Ok(_) => {
                                    println!("success!");
                                },
                                Err(e) => {
                                    println!("Error {}", e);
                                }
                            };
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
