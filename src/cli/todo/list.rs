use crate::{config::Config, store::Store};

#[derive(Debug, clap::Args)]
pub struct Args {
}

/// prints the title of every todo to stdout
pub fn run(_: &Config, store: &Store, _: &Args) {
    for todo in store.todos.iter() {
        println!("TODO: {}", todo.title);
    }
}
