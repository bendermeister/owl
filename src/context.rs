use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ContextRaw {
    todo_path: String,
    tags: Vec<String>,
}

pub struct Context {
    pub todo_path: PathBuf,
    pub tags: Vec<String>,
}

pub fn get_context() -> Result<Context, anyhow::Error> {
    let owl_dir: String = std::env::var("OWL_DIR")?;

    let mut owl_dir: PathBuf = if owl_dir.trim().is_empty() {
        std::env::current_dir()?
    } else {
        owl_dir.into()
    };

    owl_dir.push("owl.toml");
    let context_raw = std::fs::read_to_string(&owl_dir)?;
    owl_dir.pop();

    let context_raw: ContextRaw = toml::from_str(&context_raw)?;

    let mut context = Context {
        todo_path: owl_dir.clone(),
        tags: context_raw.tags,
    };

    context.todo_path.push(&context_raw.todo_path);
    Ok(context)
}
