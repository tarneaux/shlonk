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
use std::collections::HashMap;
use std::fmt::Debug;
mod config_parser;
use config_parser::{Config, Url};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, default_value = "8080")]
    port: u16,
    #[clap(short, long, default_value = "/etc/shlonk/config.yml")]
    config: String,
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let url_cache = {
        let config = Config::read(&args.config).unwrap_or_else(|e| {
            eprintln!("Error reading configuration file: {}", e);
            std::process::exit(1);
        });
        config.urls
    };
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
fn get_url(name: &str, cache: &State<HashMap<String, Url>>) -> Option<Redirect> {
    cache.get(name).map(|r| r.clone().into())
}

#[catch(404)]
fn not_found() -> &'static str {
    "Sorry, this URL was not found."
}
