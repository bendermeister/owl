use std::path::PathBuf;

use clap::Parser;
use owl::{cli, config::Config, indexer, store::Store};

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

    // TODO: delete unwrap
    let mut path: PathBuf = std::env::var("HOME").unwrap().into();
    path.push(".config");
    path.push("owl");

    path.push("config.toml");
    let config = Config::open(&path);
    path.pop();

    log::info!("read config");

    path.push("store.json");
    let mut store = Store::open(&path);
    path.pop();

    log::info!("read store");

    indexer::index(&mut store, &config);

    log::info!("scanned directories");

    let args = cli::Args::parse();

    cli::run(&config, &store, &args);

    path.push("store.json");
    store.close(&path);
}
