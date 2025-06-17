use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub store_path: PathBuf,
    pub ignore: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigRaw {
    pub store_path: Option<String>,
    pub ignore_hidden_files: Option<bool>,
    pub ignore: Option<Vec<String>>,
    pub base_directory: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    let mut path: PathBuf = std::env::var("HOME").unwrap().into();
    path.push(".config");
    path.push("owl");
    path.push("config.toml");
    path
}

fn create_default_config(path: &Path) -> Config {
    let body = r#"# DEFAULT OWL CONFIG FILE
# have fun

# store_path is the location where the internal data of owl will be stored.
store_path = "$HOME/.config/owl/store.json"

# Tells owl whether or not to ingore hidden files.
#
# In general it is recommend to leave this as true as it greatly improves index
# speed and hidden files are hidden for a reason.
#
# If you set this to false it is greatly recommended to ignore common storage 
# options of other programs via glob patterns. If you code in rust for example 
# you should add `"*/.cargo/*"` to the ignore patterns.
ignore_hidden_files = true

# ignore globs: use this to specifically exclude directories or files from
# indexing. You can also use globs and environment variables.
ignore = [
    "**/node_modules/**",
    "**/go/pkg/**",
]

# directory where the indexing should start
base_directory = "$HOME"
        
"#;
    log::info!("creating default config");

    let config_prefix = path.parent().unwrap();
    log::info!("mkdir --parents {:?}", config_prefix);
    std::fs::create_dir_all(config_prefix).unwrap();

    match std::fs::write(path, &body) {
        Ok(_) => log::info!("wrote config to {:?}", path),
        Err(e) => panic!("could not write config to: '{:?}': error: {:?}", path, e),
    };

    Config::open()
}

fn un_envvar_path(input: &str) -> Result<PathBuf, anyhow::Error> {
    let parents = input.trim().split("/");

    let mut path = PathBuf::new();

    for parent in parents {
        if let Some('$') = parent.chars().next() {
            let parent = match std::env::var(&parent[1..]) {
                Ok(var) => var,
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "could not read environment variable: {:?}: error: {:?}",
                        parent,
                        err
                    ));
                }
            };
            path.push(parent);
        } else {
            path.push(parent);
        }
    }

    Ok(path)
}

fn parse_config(body: &str) -> Result<Config, anyhow::Error> {
    let config_raw: ConfigRaw = toml::from_str(body)?;

    let store_path = match &config_raw.store_path {
        Some(path) => path.as_str(),
        None => {
            log::warn!("no store_path present in config file, using default");
            "$HOME/.config/owl/store.json"
        }
    };

    let store_path = match un_envvar_path(store_path) {
        Ok(path) => path,
        Err(err) => {
            return Err(anyhow::anyhow!(
                "could not build path to store, error: {:?}",
                err
            ));
        }
    };

    let ignore = config_raw.ignore.unwrap_or_else(|| {
        log::warn!("no ignore list present in config file, using default");
        vec![
            "*/.cargo/*".into(),
            "*/node_modules/*".into(),
            "*/go/pkg/*".into(),
            "$HOME/.config/*".into(),
            "$HOME/.local/*".into(),
        ]
    });

    let ignore: Result<Vec<Vec<u8>>, anyhow::Error> = ignore
        .into_iter()
        .map(|path| un_envvar_path(&path).map(|path| path.as_os_str().as_encoded_bytes().to_vec()))
        .collect();

    let ignore = match ignore {
        Ok(ignore) => ignore,
        Err(err) => {
            return Err(anyhow::anyhow!(
                "could not resolve environment variables in ignore list: {:?}",
                err
            ));
        }
    };

    Ok(Config { store_path, ignore })
}

impl Config {
    pub fn open() -> Self {
        let path = get_config_path();

        match std::fs::read_to_string(&path) {
            Ok(body) => {
                log::info!("read config file at: {:?}", path);
                let config: Config = match parse_config(&body) {
                    Ok(config) => config,
                    Err(e) => panic!("could not parse config at: {:?}: error: {:?}", path, e),
                };
                log::info!("parsed config file at: {:?}", path);
                config
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => create_default_config(&path),
                _ => panic!("could not read config at: {:?}: error: {:?}", path, err),
            },
        }
    }
}
