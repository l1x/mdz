use clap::{ArgMatches, Command};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build").about("Builds the site")
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    let _ = args.args_present();

    // load the state file
    // check if there is a new markdown that has not been converted yet
    // if there is convert it
    // if there isnt exit

    Ok(())
}
