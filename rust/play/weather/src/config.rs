use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use home::home_dir;
use serde::{Deserialize, Serialize};

use crate::{error::Result, APP};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    path: PathBuf,
    pub api_key: Option<String>,
    #[serde(flatten)]
    pub location: Option<Location>,
    #[serde(default)]
    pub locations: HashMap<String, Location>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: Self::get_path(Base::FromEnv),
            location: None,
            api_key: None,
            locations: HashMap::new(),
        }
    }
}

pub enum Base<'a> {
    FromEnv,
    Dir(&'a Path),
}

impl Config {
    fn get_path(basedir: Base) -> PathBuf {
        let mut path = match basedir {
            Base::Dir(dir) => dir.to_path_buf(),
            Base::FromEnv => match env::var("XDG_CONFIG_HOME") {
                Ok(dir) => Path::new(&dir).join(APP),
                Err(_) => match home_dir() {
                    Some(dir) => Path::new(&dir).join(".config").join(APP),
                    None => PathBuf::new(),
                },
            },
        };
        path = path.join(APP);
        path.set_extension("toml");

        path
    }

    pub fn load(basedir: Base) -> Result<Self> {
        let path = Config::get_path(basedir);
        let toml = std::fs::read_to_string(path)?;
        let config = match toml::from_str(&toml) {
            Ok(c) => c,
            Err(e) => panic!("{}", e.to_string()),
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn config_works() {
        let dir = tempdir().unwrap();
        let config_file = Config::get_path(Base::Dir(dir.path()));

        // `Config` adds filename to `basedir`.
        let mut filename = PathBuf::from(APP);
        filename.set_extension("toml");

        assert_eq!(config_file, dir.path().to_path_buf().join(filename));
    }
}
