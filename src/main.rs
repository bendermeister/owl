use owl::cli;
use owl::context::get_context;

fn main() {
    let context = get_context().unwrap();
    cli::run(context).unwrap();
}
