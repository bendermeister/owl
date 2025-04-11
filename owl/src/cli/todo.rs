use crate::id::ID;
use crate::store::Store;
use crate::tag::Tag;
use crate::todo::Todo;
use std::collections::HashSet;

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
    /// Title of the new todo
    title: String,

    /// tags for the new todo (comma seperated)
    #[clap(long)]
    tag: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// tags to filter for
    #[clap(long)]
    tag: Option<String>,
}

pub fn run(store: &mut Store, args: Args) {
    match args.command {
        Command::New(new_args) => run_new(store, new_args),
        Command::List(list_args) => run_list(store, list_args),
    }
}

fn run_new(store: &mut Store, args: NewArgs) {
    let mut todo = Todo::new(args.title);
    if let Some(tag) = args.tag {
        let tag = parse_tags(store, &tag);
        todo.tags = tag;
    }
    store.store_todo(todo).unwrap()
}

fn parse_tags(store: &Store, tags: &str) -> HashSet<ID<Tag>> {
    let tags: Result<HashSet<ID<Tag>>, anyhow::Error> = tags
        .trim()
        .split(",")
        .map(|t| t.trim())
        .map(|t| {
            store
                .tags
                .get(t)
                .map(|t| Ok(*t))
                .unwrap_or_else(|| Err(anyhow::anyhow!("could not find tag")))
        })
        .collect();

    let tags = tags.unwrap();
    tags
}

fn run_list(store: &mut Store, args: ListArgs) {
    let mut todos = store.get_todos().unwrap();

    // filter based on tag
    if let Some(tags) = &args.tag {
        let tags = parse_tags(store, tags);
        todos = todos
            .into_iter()
            .filter(|t| t.tags.intersection(&tags).count() > 0)
            .collect();
    }

    todos = todos.into_iter().filter(|t| t.closed.is_none()).collect();

    for todo in todos.into_iter() {
        println!("{} {} {}", todo.id, todo.title, todo.opened);
    }
}
