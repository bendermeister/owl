use clap::Parser;

use crate::context::{Context, OutputFormat};

mod agenda;
mod clocktime;
mod list;

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// desired output format
    #[clap(long)]
    #[clap(value_enum, default_value_t = OutputFormat::Colorful)]
    format: OutputFormat,

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

pub fn run(context: &mut Context) {

    let args = Args::parse();
    context.output_format = args.format;

    match args.command {
        Command::List(args) => list::run(&context, args),
        Command::Agenda(args) => agenda::run(&context, args),
    }
}
