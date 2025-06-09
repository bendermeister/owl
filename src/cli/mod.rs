use clap::Parser;
use crate::context::Context;


#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

mod todo;

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// todo commands
    Todo(todo::Args),
}

pub fn run(context: Context) -> Result<(), anyhow::Error> {
    let args = Args::parse();
    match args.command {
        Command::Todo(args) => todo::run(context, args),
    }
}
