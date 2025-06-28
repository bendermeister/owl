use crate::{config::Config, store::Store};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// should the paths be listed for the individual todos
    #[clap(long, short)]
    path: bool,
}

/// prints the title of every todo to stdout
pub fn run(_: &Config, store: &Store, args: &Args) {
    for todo in store.todos.iter() {
        if args.path {
            print!("{}:{} ", todo.path.to_str().unwrap(), todo.line_number);
        }
        print!("TODO: {}", todo.title);
        println!();
    }
}
