#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use chrono::Local;
use clap::Command;
use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::Write;

mod cmd;
pub mod common;

const VERSION: &str = concat!("v", crate_version!());

fn main() {
    init_logger();
    let command = create_clap_command();
    let res = match command.get_matches().subcommand() {
        Some(("build", sub_matches)) => cmd::build::execute(sub_matches),
        //Some(("clean", sub_matches)) => cmd::clean::execute(sub_matches),
        _ => unreachable!(),
    };

    match res {
        Ok(_) => info!("Ok"),
        Err(_) => error!("Err"),
    }
}

/// Create a list of valid arguments and sub-commands
fn create_clap_command() -> Command {
    let app = Command::new(crate_name!())
        .about(crate_description!())
        .author("Istvan Szukacs <istvan@datadeft.eu>")
        .version(VERSION)
        .propagate_version(true)
        .arg_required_else_help(true)
        .after_help(
            "For more information about a specific subcommand, run `mdz <command> --help`\n\
             The source code for mdz is available at: https://github.com/l1x/mdz",
        )
        .subcommand(cmd::build::make_subcommand());
    //.subcommand(cmd::clean::make_subcommand());

    app
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}
