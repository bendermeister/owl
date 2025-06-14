use crate::context::Context;
use crate::context::OutputFormat;
use clap::Parser;
use crate::store::Store;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    store: std::path::PathBuf,

    /// desired output format
    #[clap(long)]
    #[clap(value_enum, default_value_t = OutputFormat::Colorful)]
    format: OutputFormat,

    #[clap(subcommand)]
    command: Command,
}

mod search;
mod todo;

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// todo commands
    Todo(todo::Args),

    /// search for a phrase
    Search(search::Args),
}

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let store = Store::open(&args.store)?;

    let context = Context {
        store,
        output_format: args.format,
    };

    match args.command {
        Command::Todo(args) => todo::run(context, args),
        Command::Search(args) => search::run(context, args),
    }
}
