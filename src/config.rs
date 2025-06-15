use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub store_path: PathBuf,
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
