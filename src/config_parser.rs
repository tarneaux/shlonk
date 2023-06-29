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
    pub fn read(path: &str) -> Result<Self, ConfigReadingError> {
        let mut file = File::open(path).map_err(ConfigReadingError::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(ConfigReadingError::IoError)?;
        let config: Self =
            serde_yaml::from_str(&contents).map_err(ConfigReadingError::YamlError)?;
        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    #[serde(default = "default_permanent")]
    pub permanent: bool,
}

impl From<Url> for Redirect {
    fn from(url: Url) -> Self {
        if url.permanent {
            Self::permanent(url.url)
        } else {
            Self::to(url.url)
        }
    }
}

#[derive(Debug)]
pub enum ConfigReadingError {
    IoError(std::io::Error),
    YamlError(serde_yaml::Error),
}

impl Display for ConfigReadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::YamlError(e) => write!(f, "YAML error: {}", e),
        }
    }
}

const fn default_port() -> u16 {
    8080
}

const fn default_permanent() -> bool {
    false
}
