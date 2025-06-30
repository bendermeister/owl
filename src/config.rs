use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub ignore_hidden_files: bool,
    pub ignore: Vec<Vec<u8>>,
    pub base_directory: PathBuf,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct ConfigRaw {
    pub ignore_hidden_files: bool,
    pub ignore: Vec<String>,
    pub base_directory: String,
}

fn unenvar_path(path: &str) -> String {
    let mut buffer = String::new();

    for part in path.trim().split("/") {
        if let Some(part) = part.strip_prefix("$") {
            let part = match std::env::var(part) {
                Ok(part) => part,
                Err(err) => panic!(
                    "could not resolve environment varialbe in config '${}' because: {:?}",
                    part, err
                ),
            };
            buffer.push_str(&part);
        } else {
            buffer.push_str(part);
        }
        buffer.push('/');
    }

    buffer.pop();
    buffer
}

impl Config {
    /// creates a default config at path and returns it
    ///
    /// # Panics
    /// if an error occures this function panics with a message
    pub fn create_default(path: &Path) -> Self {
        let default_body = r#"# Default Owl Config

# base directory from which the scan for todos and tasks starts
base_directory = "$HOME"

# list of glob patterns which should be ignored in the scan. this is very useful for config files,
# external things which are out of your control (eg. node_modules directory). ignore globs may also
# include environment variables
ignore = [
    "**/go/pkg/**",
    "**/node_modules/**",
    "**/.cargo/**",
    "**/target/debug/**",
    "**/target/release/**",
    "$HOME/.config/**",
]

# whether or not hidden files and directories should be ignored
ignore_hidden_files = true
"#;
        assert!(path.is_absolute());

        // this unwrap should be safe because no relative path should every be fed here
        let parent = path.parent().unwrap();

        match std::fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(err) => panic!(
                "could not create defaul config at: {:?} because parent directories could not be created because: {:?}",
                path, err
            ),
        };

        log::info!("created parent directories of {:?}", path);

        match std::fs::write(path, default_body) {
            Ok(_) => (),
            Err(err) => panic!(
                "could not create default config at: {:?} because: {:?}",
                path, err
            ),
        };

        log::info!("wrote default config to: {:?}", path);

        Self::open(path)
    }

    /// opens the config located at path and deserializes it from toml if the file does not exist
    /// the `Config::create_default` function gets called to create it
    ///
    /// # Panics
    /// if an error occures this fucntion panics with a message
    pub fn open(path: &Path) -> Self {
        let config = match std::fs::read_to_string(path) {
            Ok(config) => config,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return Self::create_default(path),
                _ => panic!("could not read config at: {:?} because: {:?}", path, err),
            },
        };

        log::info!("read config body from {:?}", path);

        let config: ConfigRaw = match toml::from_str(&config) {
            Ok(config) => config,
            Err(err) => panic!(
                "could not deserialize config from toml at: {:?} because {:?}",
                path, err
            ),
        };

        log::info!("deserialized config from: {:?}", path);

        let ignore_hidden_files = config.ignore_hidden_files;

        let ignore = config
            .ignore
            .iter()
            .map(|path| unenvar_path(path).as_bytes().to_vec())
            .collect();
        log::info!("config: resolved environment variables in ignore");

        let base_directory = unenvar_path(&config.base_directory).into();
        log::info!("config: resolved environment variables in base_directory");

        Config {
            ignore_hidden_files,
            ignore,
            base_directory,
        }
    }
}
