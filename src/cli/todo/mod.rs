use crate::context::Context;

mod agenda;
mod list;
mod clocktime;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// todo commands
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// List all current todos
    List(list::Args),

    /// List todos sorted and grouped by there scheduled time and deadline
    Agenda(agenda::Args),
}

pub fn run(context: &Context, args: Args) -> Result<(), anyhow::Error> {
    match args.command {
        Command::List(args) => list::run(context, args),
        Command::Agenda(args) => agenda::run(context, args),
    }
}
