use clap::{ArgMatches, Command};
use std::{
    env, error,
    fs::{self},
    path::{Path, PathBuf},
};

use crate::common::file::{get_md_files, is_file};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build").about("Builds the site")
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let _ = args.args_present();

    // load the state file
    // check if there is a new markdown that has not been converted yet
    // if there is convert it
    // if there isnt exit

    // load config and read input dir

    // list input files
    info!("{:?}", env::current_dir());
    let absolute_path = Path::new(&env::current_dir().unwrap()).join("in");
    info!("{:?}", absolute_path);
    let md_files = get_md_files(absolute_path.to_str());
    info!("{:?}", md_files);
    // render plan
    // render
    // checksum verification

    println!("{}", markdown::to_html("## Hello, *world*!"));

    Ok(())
}
