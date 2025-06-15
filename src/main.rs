use owl::cli;
use owl::config::Config;
use owl::context::{Context, OutputFormat};
use owl::indexer;
use owl::store::Store;
use std::path::PathBuf;

fn main() {
    let home_directory: PathBuf = std::env::var("HOME").unwrap().into();
    let config = Config::open();
    let mut store = Store::open(&config.store_path).unwrap();
    indexer::index(&mut store, home_directory.as_path()).unwrap();

    let context = Context {
        store,
        config,
        output_format: OutputFormat::Colorful,
    };

    cli::run(context).unwrap();
}
