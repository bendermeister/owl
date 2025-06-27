use clap::Parser;

use crate::{config::Config, store::Store};

mod agenda;
mod task;
mod todo;

#[derive(Parser, Debug)]
pub struct Args {
    /// all supported subcommands
    #[clap(subcommand)]
    command: Command,
}


#[derive(Debug, clap::Subcommand)]
enum Command {
    /// todo subcommand
    Todo(todo::Args),

    /// agenda subcommand
    Agenda(agenda::Args),

    /// task subcommand
    Task(task::Args),
}

pub fn run(config: &Config, store: &Store, args: &Args) {
    match &args.command {
        Command::Todo(args) => todo::run(config, store, args),
        Command::Agenda(args) => agenda::run(config, store, args),
        Command::Task(args) => task::run(config, store, args),
    }
}
