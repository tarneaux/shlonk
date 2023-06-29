/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

#[macro_use]
extern crate rocket;
use rocket::{get, response::Redirect, routes, State};
mod config_parser;
use config_parser::Config;
mod argument_parser;
use argument_parser::Args;
use clap::Parser;

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let config = Config::read(&args.config).unwrap_or_else(|e| {
        eprintln!("Error reading configuration file: {}", e);
        std::process::exit(1);
    });
    let rocket_config = rocket::Config {
        port: config.port,
        ..Default::default()
    };
    rocket::build()
        .manage(config)
        .mount("/", routes![index])
        .mount("/", routes![get_url])
        .register("/", catchers![not_found])
        .configure(rocket_config)
}

#[get("/<name>")]
fn get_url(name: &str, config: &State<Config>) -> Option<Redirect> {
    config.urls.get(name).map(|r| r.clone().into())
}

#[get("/")]
const fn index() -> &'static str {
    concat!(
        "Welcome to Shlonk, a simple, fast, URL shortener.\n",
        "There is nothing to see here, please go to /<name> to get redirected to the URL you want.\n",
        "If you are the owner of this Shlonk instance, edit the YAML file to add new URLs.\n",
    )
}

#[catch(404)]
const fn not_found() -> &'static str {
    "Sorry, this URL was not found."
}
