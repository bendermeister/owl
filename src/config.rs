use std::io;
use std::path::PathBuf;

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

impl Config {
    pub fn open() -> Self {
        let path = get_config_path();
        let config = match std::fs::read_to_string(&path) {
            Ok(body) => Some(body),
            Err(err) => match err.kind() {
                io::ErrorKind::PermissionDenied => panic!(
                    "You don't have necessary permissions to read your own config! What went wrong? config at: {:?}",
                    path
                ),
                io::ErrorKind::NotFound => None,
                _ => panic!("Could not read config file at: {:?}", path),
            },
        };

        if let Some(config) = config {
            toml::from_str(&config).unwrap()
        } else {
            let config = Config::default();
            let config_body = toml::to_string_pretty(&config).unwrap();
            let config_prefix = path.parent().unwrap();
            std::fs::create_dir_all(config_prefix).unwrap();
            std::fs::write(path, &config_body).unwrap();
            config
        }
    }
}
