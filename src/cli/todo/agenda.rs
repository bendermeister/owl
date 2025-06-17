use crate::context::Context;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long)]
    range: Option<String>,

    #[clap(long)]
    interval: Option<String>,
}

pub fn run(context: &Context, args: Args) -> Result<(), anyhow::Error> {
    todo!()
}
