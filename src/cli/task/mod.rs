use crate::{config::Config, store::Store};

mod list;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// lists every task
    List(list::Args),
}

pub fn run(config: &Config, store: &Store, args: &Args) {
    match &args.command {
        Command::List(args) => list::run(config, store, args),
    }
}
