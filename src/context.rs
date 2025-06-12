use crate::db;
use crate::indexer;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ContextRaw {
    tags: Vec<String>,
}

#[derive(Debug)]
pub struct Context {
    pub tags: Vec<String>,
    pub db: rusqlite::Connection,
}

pub fn get_context(mut owl_dir: PathBuf) -> Result<Context, anyhow::Error> {
    owl_dir.push("owl.toml");
    let context_raw = std::fs::read_to_string(&owl_dir)?;
    let context_raw: ContextRaw = toml::from_str(&context_raw)?;
    owl_dir.pop();

    owl_dir.push("owl.sqlite");
    let db = rusqlite::Connection::open(&owl_dir)?;
    db::migration::migrate(&db)?;
    owl_dir.pop();

    indexer::index(&db, std::fs::read_dir(&owl_dir).unwrap())?;

    let context = Context {
        tags: context_raw.tags,
        db: db,
    };

    Ok(context)
}
