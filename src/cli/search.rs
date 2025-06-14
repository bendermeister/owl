use crate::context::Context;
use crate::tfidf;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// phrase to search for
    phrase: String,
}

pub fn run(context: Context, args: Args) -> Result<(), anyhow::Error> {
    let paths = tfidf::rank(&context.store, &args.phrase);
    for path in paths {
        println!("{:?}", path);
    }
    Ok(())
}
