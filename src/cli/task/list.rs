use crate::{config::Config, store::Store};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long, short)]
    path: bool,
}

/// prints the title of every task to stdout
pub fn run(_: &Config, store: &Store, args: &Args) {
    for task in store.tasks.iter() {
        if args.path {
            print!("{}:{} ", task.path.to_str().unwrap(), task.line_number);
        }
        print!("TASK: {}", task.title);
        println!();
    }
}
