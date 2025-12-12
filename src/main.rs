use std::path::PathBuf;
use structopt::StructOpt;

mod utils;

#[derive(StructOpt, Debug)]
struct Forge {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
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
    }
    println!("{:?}", opt.command);
}
