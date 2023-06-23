/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

/*
 * This module parses the YAML configuration file using serde_yaml.
 * This file contains a list of URLS in the following format:
 * urls:
 *   - name: example
 *     url: https://example.com
 *     permanent: false # optional
 *  ...
 * It also contains the following options:
 * port: 8080 # optional
 * error_page: /usr/share/shlonk/404.html # optional
 * In the above example, all fields that are marked as optional are set to their default value.
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
    #[serde(default = "default_error_page")]
    pub error_page: String,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, ConfigReadingError> {
        let mut file = File::open(path).map_err(|e| ConfigReadingError::IoError(e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| ConfigReadingError::IoError(e))?;
        let config: Config =
            serde_yaml::from_str(&contents).map_err(|e| ConfigReadingError::YamlError(e))?;
        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    // pub name: String, // TODO: make a default function fo the name: we can infer it from the end of the url
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
pub enum ConfigReadingError {
    IoError(std::io::Error),
    YamlError(serde_yaml::Error),
}

impl Display for ConfigReadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigReadingError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigReadingError::YamlError(e) => write!(f, "YAML error: {}", e),
        }
    }
}

fn default_port() -> u16 {
    8080
}

fn default_error_page() -> String {
    "/usr/share/shlonk/404.html".to_string()
}

fn default_permanent() -> bool {
    false
}
