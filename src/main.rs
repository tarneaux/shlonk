/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

/*
 * This program takes a list of redirections from a configuration file in this format:
 * <name> <url> [permanent]
 * - name is the name of the path
 * - url is the URL to redirect to
 * - permanent is an optional argument, if present, the redirection will be permanent (301) instead of temporary (302)
 * Comments are supported and start with a #.
 * Example:
 * google https://google.com # Redirects /google to https://google.com
 * github https://github.com permanent # Redirects /github to https://github.com with a 301
 * The configuration file is located at /etc/shlonk.conf by default, but can be changed with the -c argument.
 */

/*
 * Command-line arguments:
 * - -p <port> to specify the port to listen on (default: 8080)
 * - -h to display help
 * - -c to set the configuration file (default: /etc/shlonk.conf)
 */

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::{get, response::Redirect, routes, State};
use std::fmt::{Debug, Display, Formatter};
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, default_value = "8080")]
    port: u16,
    #[clap(short, long, default_value = "/etc/shlonk.conf")]
    config: String,
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let url_cache = get_urls(&args.config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let config = rocket::Config {
        port: args.port,
        ..Default::default()
    };
    rocket::build()
        .manage(url_cache)
        .mount("/", routes![get_url])
        .register("/", catchers![not_found])
        .configure(config)
}

#[get("/<name>")]
fn get_url(name: &str, cache: &State<HashMap<String, MyRedirect>>) -> Option<Redirect> {
    cache.get(name).map(|r| r.clone().into())
}

#[catch(404)]
fn not_found() -> &'static str {
    "Sorry, this URL was not found."
}

#[derive(Debug)]
enum ConfigParseError {
    LineParseError(LineParseError, String),
    IoError(std::io::Error),
}

impl Display for ConfigParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigParseError::LineParseError(e, line) => {
                write!(
                    f,
                    "Error parsing line \"{}\" in configuration file: {}",
                    line, e
                )
            }
            ConfigParseError::IoError(e) => write!(f, "Error reading configuration file: {}", e),
        }
    }
}

#[derive(Debug)]
enum LineParseError {
    MissingName,
    MissingUrl,
    InvalidPermanent,
}

impl Display for LineParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineParseError::MissingName => write!(f, "Missing name"),
            LineParseError::MissingUrl => write!(f, "Missing URL"),
            LineParseError::InvalidPermanent => write!(f, "Invalid permanent argument"),
        }
    }
}

fn get_urls(path: &str) -> Result<HashMap<String, MyRedirect>, ConfigParseError> {
    let mut file = File::open(path).map_err(|e| ConfigParseError::IoError(e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ConfigParseError::IoError(e))?;
    parse_all_urls(contents)
}

fn parse_all_urls(contents: String) -> Result<HashMap<String, MyRedirect>, ConfigParseError> {
    let mut urls = HashMap::new();
    for line in contents.lines() {
        parse_line(line, &mut urls)
            .map_err(|e| ConfigParseError::LineParseError(e, line.to_string()))?;
    }
    Ok(urls)
}

fn parse_line(
    line: &str,
    redirections: &mut HashMap<String, MyRedirect>,
) -> Result<(), LineParseError> {
    if line.starts_with('#') || line.is_empty() {
        return Ok(());
    }
    let mut split = line.split(' ');
    let path = split.next().ok_or(LineParseError::MissingName)?;
    let url = split.next().ok_or(LineParseError::MissingUrl)?;
    let permanent = match split.next() {
        Some("permanent") => Ok(true),
        Some(_) => Err(LineParseError::InvalidPermanent),
        None => Ok(false),
    }?;
    let redirect = match permanent {
        true => MyRedirect::permanent(url),
        false => MyRedirect::temporary(url),
    };
    redirections.insert(path.to_string(), redirect);
    Ok(())
}

// Rocket doesn't want us to clone their Redirect type.
// No problem, we'll just make our own, and convert it to a Redirect when needed.
#[derive(Clone)]
struct MyRedirect {
    url: String,
    permanent: bool,
}

impl MyRedirect {
    fn permanent(url: &str) -> Self {
        Self {
            url: url.to_string(),
            permanent: true,
        }
    }

    fn temporary(url: &str) -> Self {
        Self {
            url: url.to_string(),
            permanent: false,
        }
    }
}

impl Into<Redirect> for MyRedirect {
    fn into(self) -> Redirect {
        if self.permanent {
            Redirect::permanent(self.url)
        } else {
            Redirect::temporary(self.url)
        }
    }
}
