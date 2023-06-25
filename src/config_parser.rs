/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

/*
 * This module parses the YAML configuration file using serde_yaml.
 * This file contains a list of URLS in the following format:
 * urls:
 *   example:
 *     url: https://example.com
 *     permanent: false # optional, defaults to false
 *  ...
 * It also contains the following options:
 * port: 8080 # optional, defaults to 8080
 */

use rocket::response::Redirect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub urls: HashMap<String, Url>,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, ConfigError> {
        let mut file = File::open(path).map_err(|e| ConfigError::IoError(e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| ConfigError::IoError(e))?;
        let config: Config =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::YamlError(e))?;
        println!("Read config file: {:?}", config);
        Ok(config)
    }
    pub fn write(&self, path: &str) -> Result<(), ConfigError> {
        let mut file = File::create(path).map_err(|e| ConfigError::IoError(e))?;
        let contents = serde_yaml::to_string(self).map_err(|e| ConfigError::YamlError(e))?;
        file.write_all(contents.as_bytes())
            .map_err(|e| ConfigError::IoError(e))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    #[serde(default = "default_permanent")]
    pub permanent: bool,
}

impl Into<Redirect> for Url {
    fn into(self) -> Redirect {
        if self.permanent {
            Redirect::permanent(self.url)
        } else {
            Redirect::temporary(self.url)
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    YamlError(serde_yaml::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::YamlError(e) => write!(f, "YAML error: {}", e),
        }
    }
}

fn default_port() -> u16 {
    8080
}

fn default_permanent() -> bool {
    false
}
