use crate::store::Store;

#[derive(Debug)]
pub struct Context {
    pub store: Store, 
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Colorful,
    Plain,
    Json,
}
