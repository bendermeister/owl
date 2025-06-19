use owl::cli;
use owl::config::Config;
use owl::context::{Context, OutputFormat};
use owl::indexer;
use owl::store::Store;

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

    let config = Config::open();

    let mut store = Store::open(&config.store_path);

    indexer::index(&config, &mut store, config.base_directory.as_path());

    let mut context = Context {
        store,
        config,
        output_format: OutputFormat::Colorful,
    };

    // TODO: log this
    cli::run(&mut context).unwrap();

    context.store.close(&context.config.store_path).unwrap();
}
