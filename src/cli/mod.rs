use crate::context;
use crate::context::OutputFormat;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
struct Args {
    /// base directory from where to start the indexing
    base_directory: String,

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
    let mut context = context::get_context(args.base_directory.into())?;
    context.output_format = args.format;
    match args.command {
        Command::Todo(args) => todo::run(context, args),
        Command::Search(args) => search::run(context, args),
    }
}
