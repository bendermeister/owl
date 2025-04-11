// use clap::{Args, Parser, Subcommand};
use crate::store::Store;
use clap::Parser;
use std::path::{Path, PathBuf};

mod tag;
mod todo;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
struct Args {
    pub directory: Option<PathBuf>,

    /// commands
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// todo commands
    Todo(todo::Args),

    /// tag commands
    Tag(tag::Args),

    /// Sync files with database
    Update(UpdateArgs),
}

#[derive(Debug, clap::Args)]
struct UpdateArgs {}

pub fn init_store(cwd: Option<PathBuf>) -> Store {
    let cwd = match cwd {
        Some(path) => path,
        None => std::env::current_dir().unwrap(),
    };
    let cwd = std::path::absolute(&cwd).unwrap();
    Store::open(cwd).unwrap()
}

pub fn run() {
    let args = Args::parse();
    let mut store = init_store(args.directory);
    store.update(false).unwrap();

    match args.command {
        Command::Todo(args) => todo::run(&mut store, args),
        Command::Tag(args) => tag::run(&mut store, args),
        Command::Update(_) => store.update(true).unwrap(),
    }
}
