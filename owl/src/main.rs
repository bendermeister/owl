use clap::{Args, Parser, Subcommand};
use owl::store::Store;
use owl::todo::Todo;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Arguments {
    /// which command to run
    #[clap(subcommand)]
    pub command: Command,

    /// where to search for files
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// do something with todos
    Todo(TodoArguments),

    /// update the database
    Update,
}

#[derive(Debug, Args)]
struct TodoArguments {
    #[clap(subcommand)]
    command: TodoSubCommand,
}

#[derive(Debug, Subcommand, Clone)]
enum TodoSubCommand {
    /// add a new todo
    New(TodoNewArguments),
    List,
}

#[derive(Debug, Args, Clone)]
struct TodoNewArguments {
    /// Title of the todo
    pub title: String,
}

fn todo_command(store: &mut Store, args: TodoArguments) {
    match args.command {
        TodoSubCommand::New(args) => store.store(Todo::new(args.title)).unwrap(),
        TodoSubCommand::List => {
            let todos: Vec<_> = store
                .get_todos()
                .unwrap()
                .into_iter()
                .filter(|t| t.closed.is_none())
                .collect();

            for todo in todos {
                println!("{}\t{} {}", todo.id, todo.title, todo.opened);
            }
        }
    }
}

fn main() {
    let mut args = Arguments::parse();

    // TODO: better error management
    if args.directory.is_none() {
        args.directory = Some(std::env::current_dir().unwrap());
    }

    let cwd = match args.directory {
        Some(dir) => dir,
        None => std::env::current_dir().unwrap(),
    };

    let cwd = std::path::absolute(&cwd).unwrap();
    let mut store = Store::open(cwd).unwrap();

    match args.command {
        Command::Todo(args) => todo_command(&mut store, args),
        Command::Update => {
            store.update().unwrap();
        }
    }
}
