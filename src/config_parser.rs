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
 * token: secret # optional.
 * If token is not set, changes to URLs will not be allowed.
 * If token is set to anything else, it will be used as the token.
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
    pub token: Option<String>,
    #[serde(skip)]
    pub path: String,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, ConfigError> {
        let mut file = File::open(path).map_err(ConfigError::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(ConfigError::IoError)?;
        let mut config: Self = serde_yaml::from_str(&contents).map_err(ConfigError::YamlError)?;
        println!("Read config file: {:?}", config);
        config.path = path.to_string();
        Ok(config)
    }
    pub fn write(&self) -> Result<(), ConfigError> {
        let mut file = File::create(self.path.clone()).map_err(ConfigError::IoError)?;
        let contents = serde_yaml::to_string(self).map_err(ConfigError::YamlError)?;
        file.write_all(contents.as_bytes())
            .map_err(ConfigError::IoError)?;
        Ok(())
    }
    pub fn authorized(&self, token: Option<String>, authlevel: AuthLevel) -> bool {
        match authlevel {
            AuthLevel::Read => true,
            AuthLevel::Add => self.token == token,
            AuthLevel::Modify => self.token == token,
            AuthLevel::Delete => self.token == token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    #[serde(default = "default_permanent")]
    pub permanent: bool,
}

impl From<Url> for Redirect {
    fn from(val: Url) -> Self {
        if val.permanent {
            Self::permanent(val.url)
        } else {
            Self::temporary(val.url)
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
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::YamlError(e) => write!(f, "YAML error: {}", e),
        }
    }
}

pub enum AuthLevel {
    Read,
    Add,
    Modify,
    Delete,
}

const fn default_port() -> u16 {
    8080
}

const fn default_permanent() -> bool {
    false
}
