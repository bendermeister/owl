use crate::{config::Config, store::Store};

#[derive(Debug, clap::Args)]
pub struct Args {
}

/// prints the title of every task to stdout
pub fn run(_: &Config, store: &Store, _: &Args) {
    for task in store.tasks.iter() {
        println!("TODO: {}", task.title);
    }
}
