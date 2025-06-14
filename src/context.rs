use crate::db;
use crate::indexer;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub db: rusqlite::Connection,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Colorful,
    Plain,
    Json,
}

pub fn get_context(mut owl_dir: PathBuf) -> Result<Context, anyhow::Error> {
    owl_dir.push("owl.sqlite");
    let db = rusqlite::Connection::open(&owl_dir)?;
    db::migration::migrate(&db)?;
    owl_dir.pop();

    indexer::index(&db, std::fs::read_dir(&owl_dir).unwrap())?;

    let context = Context {
        db,
        output_format: OutputFormat::Colorful,
    };

    Ok(context)
}
