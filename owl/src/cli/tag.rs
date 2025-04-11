use crate::id::ID;
use crate::store::Store;
use crate::tag::Tag;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// todo commands
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// create a new todo
    New(NewArgs),

    /// list existing todos
    List(ListArgs),
}

#[derive(Debug, clap::Args)]
pub struct NewArgs {
    /// Name of the new tag
    name: String,
}

#[derive(Debug, clap::Args)]
pub struct ListArgs {}

fn run_new(store: &mut Store, args: NewArgs) {
    let tag = Tag::new(ID::generate(), args.name);
    store.store_tag(tag).unwrap();
}

fn run_list(store: &mut Store, _: ListArgs) {
    for (tag, _) in store.tags.iter() {
        println!("{}", tag);
    }
}

pub fn run(store: &mut Store, args: Args) {
    match args.command {
        Command::New(new_args) => run_new(store, new_args),
        Command::List(list_args) => run_list(store, list_args),
    }
}
