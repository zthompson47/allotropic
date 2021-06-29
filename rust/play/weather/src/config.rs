use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use home::home_dir;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::{
    error::{err, Result},
    APP,
};

#[derive(Debug, Default, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(skip)]
    config: Option<Config>,
    /// Hourly forecast
    #[structopt(short, conflicts_with = "daily")]
    pub hourly: bool,
    /// Daily forecast (default)
    #[structopt(short, conflicts_with = "hourly")]
    pub daily: bool,
    /// Latitude
    #[structopt(env, long = "lat")]
    pub latitude: Option<f64>,
    /// Longitude
    #[structopt(env, long = "lon")]
    pub longitude: Option<f64>,
    /// Api key
    #[structopt(env, short = "k")]
    pub api_key: Option<String>,
    /// Profile
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
    #[serde(skip)]
    path: PathBuf,
    pub resolution: Option<Resolution>,
    pub api_key: Option<String>,
    #[serde(flatten)]
    pub location: Option<Location>,
    #[serde(default)]
    pub locations: HashMap<String, Location>,
}

#[derive(Debug)]
pub struct Params {
    pub latitude: f64,
    pub longitude: f64,
    pub resolution: Resolution,
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
    Daily,
    Hourly,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: Self::get_path(Base::FromEnv),
            resolution: None,
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

impl Params {
    pub fn from_merge(config: &Config, opt: &Opt) -> Result<Self> {
        let (latitude, longitude) = if let Some(l) = opt.location.as_ref() {
            if let Some(l) = config.locations.get(l) {
                (l.latitude, l.longitude)
            } else {
                return err("Please provide a latitude/longitude location");
            }
        } else if opt.latitude.is_some() && opt.longitude.is_some() {
            (opt.latitude.unwrap(), opt.longitude.unwrap())
        } else if let Some(l) = config.location.as_ref() {
            (l.latitude, l.longitude)
        } else {
            return err("Please provide a latitude/longitude location");
        };

        let api_key = match opt.api_key.as_ref() {
            Some(k) => k,
            None => match config.api_key.as_ref() {
                Some(k) => k,
                None => return err("Please provide an api key"),
            },
        };

        let resolution = match opt.hourly {
            true => &Resolution::Hourly,
            false => match opt.daily {
                true => &Resolution::Daily,
                false => match config.resolution.as_ref() {
                    Some(res) => res,
                    None => &Resolution::Daily,
                },
            },
        };

        Ok(Params {
            latitude,
            longitude,
            api_key: api_key.clone(),
            resolution: (*resolution).clone(),
        })
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

    #[test]
    fn merge_params() {
        let dir = tempdir().unwrap();
        let config = Config::load(Base::Dir(dir.path())).unwrap_or_default();
        assert_eq!(config, Config::default());
        let opt = Opt::from_iter([APP]);
        assert_eq!(opt, Opt::default());
        let params = Params::from_merge(&config, &opt);
        assert!(params.is_err());
    }
}
