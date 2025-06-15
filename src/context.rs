use crate::store::Store;
use crate::config::Config;

#[derive(Debug)]
pub struct Context {
    pub store: Store, 
    pub config: Config,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Colorful,
    Plain,
    Json,
}
