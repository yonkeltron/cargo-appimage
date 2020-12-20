use clap::{
  app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use color_eyre::eyre::{eyre, Result};
use paris::Logger;

mod application_definition;
mod commands;
mod desktop_file;

use async_std::task;

fn main() -> Result<()> {
  color_eyre::install()?;

  let matches = app_from_crate!()
    .arg(
      Arg::with_name("linuxdeploy")
        .short("l")
        .long("linuxdeploy-path")
        .value_name("PATH")
        .takes_value(true)
        .required(false)
        .global(true),
    )
    .subcommand(
      SubCommand::with_name("init")
        .version(crate_version!())
        .author(crate_authors!())
        .about("set up local workspace for building an AppImage"),
    )
    .get_matches();

  let mut logger = Logger::new();
  let application_definition = application_definition::ApplicationDefinition::new_from_guess()?;
  match matches.subcommand() {
    ("init", Some(_init_matches)) => {
      task::block_on(commands::init::execute(application_definition))
    }
    (other, _) => {
      logger.error("Unknown subcommand");
      Err(eyre!("Unknown subcommand: {}", other))
    }
  }
}
