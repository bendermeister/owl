use owl::cli;
use owl::config::Config;

fn main() {
    let config = Config::open();
    cli::run(config).unwrap();
}
