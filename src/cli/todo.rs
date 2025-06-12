use crate::context::Context;
use crate::table;
use crate::todo;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// todo commands
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// List all current todos
    List(ListArgs),
}

#[derive(Debug, clap::Args)]
struct ListArgs {}

pub fn run(context: Context, args: Args) -> Result<(), anyhow::Error> {
    match args.command {
        Command::List(args) => run_list(context, args),
    }
}

fn run_list(context: Context, _: ListArgs) -> Result<(), anyhow::Error> {
    todo!()
}
