use owl::cli;
use owl::config::Config;
use owl::context::{Context, OutputFormat};
use owl::indexer;
use owl::store::Store;
use std::path::PathBuf;

fn logger_init() {
    let level = if let Ok(level) = std::env::var("LOG_LEVEL") {
        match level.as_str() {
            "info" => log::Level::Info,
            "warn" => log::Level::Warn,
            "error" => log::Level::Error,
            "trace" => log::Level::Trace,
            _ => panic!(
                "'{:?}' is not a valid log level. valid levels are: ['info', 'warn', 'error', 'trace']",
                level
            ),
        }
    } else {
        log::Level::Error
    };

    simple_logger::init_with_level(level).unwrap()
}

fn main() {
    logger_init();

    let home_directory: PathBuf = match std::env::var("HOME") {
        Ok(path) => path.into(),
        Err(e) => panic!("could not read $HOME environment variable: error: {:?}", e),
    };
    log::info!("read $HOME environment variable");

    let config = Config::open();

    let mut store = Store::open(&config.store_path);

    // TODO: log this
    indexer::index(&mut store, home_directory.as_path());

    let context = Context {
        store,
        config,
        output_format: OutputFormat::Colorful,
    };


    // todo: log this
    cli::run(context).unwrap();
}
