/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

#[macro_use]
extern crate rocket;
use rocket::{get, response::Redirect, routes, State};
use std::collections::HashMap;
mod config_parser;
use config_parser::{Config, Url};
mod argument_parser;
use argument_parser::Args;
use clap::Parser;

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
