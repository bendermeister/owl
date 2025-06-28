use fast_glob::glob_match;

use crate::{config::Config, store::Store};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long, short)]
    path: bool,

    #[clap(long)]
    prefix: Option<String>,

    #[clap(long)]
    glob: Option<String>,
}

/// prints the title of every task to stdout
pub fn run(_: &Config, store: &Store, args: &Args) {
    let mut tasks = store.tasks.iter().collect::<Vec<_>>();

    if let Some(prefix) = &args.prefix {
        tasks.retain(|task| task.prefix.starts_with(prefix));
    }

    if let Some(glob) = &args.glob {
        tasks.retain(|task| glob_match(glob.as_bytes(), task.prefix.as_bytes()));
    }

    for task in tasks.iter() {
        if args.path {
            print!("{}:{} ", task.path.to_str().unwrap(), task.line_number);
        }
        print!("TASK: {}: {}", task.prefix, task.title);
        println!();
    }
}
