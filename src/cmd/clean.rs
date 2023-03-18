use clap::{ArgMatches, Command};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("clean").about("Removes local state")
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    let _ = args.args_present();
    // remove the state file and the public folder
    Ok(())
}
