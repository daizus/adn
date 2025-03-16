mod actions;
mod cli;
mod config;
mod system;

use crate::cli::{Cli, Command};
use crate::config::resolve_config_path;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Preview { config_path } => {
            let path = resolve_config_path(config_path);
            crate::actions::preview(&path);
        }
        Command::Apply { config_path } => {
            let path = resolve_config_path(config_path);
            crate::actions::apply(&path);
        }
        Command::List { config_path } => {
            let path = resolve_config_path(config_path);
            crate::actions::list(&path);
        }
        Command::Status { config_path } => {
            let path = resolve_config_path(config_path);
            crate::actions::status(&path);
        }
    }
}
