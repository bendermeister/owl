use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub store_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigRaw {
    pub store_path: Option<String>,
    pub ignore_hidden_files: Option<bool>,
    pub ignore: Option<Vec<String>>,
    pub base_directory: Option<String>,
}

impl Default for ConfigRaw {
    fn default() -> Self {
        ConfigRaw {
            store_path: Some("$HOME/.config/owl/store.json".into()),
            ignore_hidden_files: Some(true),
            ignore: Some(vec![
                "*/.cargo/*".into(),
                "*/node_modules/*".into(),
                "*/go/pkg/*".into(),
                "$HOME/.config/*".into(),
                "$HOME/.local/*".into(),
            ]),
            base_directory: Some("$HOME".into()),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path: PathBuf = std::env::var("HOME").unwrap().into();
    path.push(".config");
    path.push("owl");
    path.push("config.toml");
    path
}

impl Default for Config {
    fn default() -> Self {
        let mut store_path = get_config_path();
        store_path.pop();
        store_path.push("store.json");
        Self { store_path }
    }
}

fn create_default_config(path: &Path) -> Config {
    let body = r#"# DEFAULT OWL CONFIG FILE
# have fun

# `store_path` is the location where the internal data of owl will be stored.
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
    "*/.cargo/*",
    "*/node_modules/*",
    "*/go/pkg/*",
    "$HOME/.config/*",
    "$HOME/.local/*",
]

# directory where the indexing should start
base_directory = "$HOME"
        
"#;
    let config = Config::default();
    log::info!("creating default config with values: {:?}", &config);

    let config_body = match toml::to_string_pretty(&config) {
        Ok(body) => body,
        Err(e) => panic!("could not serialize default config: error: {:?}", e),
    };

    let config_prefix = path.parent().unwrap();

    log::info!("mkdir --parents {:?}", config_prefix);
    std::fs::create_dir_all(config_prefix).unwrap();

    match std::fs::write(path, &config_body) {
        Ok(_) => log::info!("wrote config to {:?}", path),
        Err(e) => panic!("could not write config to: '{:?}': error: {:?}", path, e),
    };
    config
}

impl Config {
    pub fn open() -> Self {
        let path = get_config_path();

        if let Ok(body) = std::fs::read_to_string(&path) {
            log::info!("read config file at: {:?}", path);
            let config: Config = match toml::from_str(&body) {
                Ok(config) => config,
                Err(e) => panic!("could not parse config at: {:?}: error: {:?}", path, e),
            };
            log::info!("parsed config file at: {:?}", path);
            return config;
        } else {
            return create_default_config(&path);
        }
    }
}
