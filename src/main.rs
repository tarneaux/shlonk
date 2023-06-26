/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, response::Redirect, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
mod config_parser;
use config_parser::{AuthLevel, Config, Url};
mod argument_parser;
use argument_parser::Args;
use clap::Parser;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
        .manage(Arc::new(RwLock::new(config)))
        .mount("/", routes![get_url])
        .mount("/api", routes![api_get, api_post, api_put, api_delete])
        .register("/", catchers![not_found])
        .configure(rocket_config)
}

#[get("/<name>")]
fn get_url(name: &str, config: &State<Arc<RwLock<Config>>>) -> Result<Option<Redirect>, Status> {
    Ok(config
        .read()
        .map_err(|e| {
            eprintln!("Error reading configuration file: {}", e);
            Status::InternalServerError
        })?
        .urls
        .get(name)
        .map(|r| r.clone().into()))
}

#[get("/<name>", data = "<data>")]
fn api_get(
    name: String,
    data: Json<ApiRequestData>,
    config: &State<Arc<RwLock<Config>>>,
) -> Result<String, Status> {
    read_config(data.token.clone(), config, AuthLevel::Read)?
        .urls
        .get(&name)
        .map_or_else(|| Err(Status::NotFound), |r| Ok(r.url.clone()))
}

#[post("/<name>", data = "<data>")]
fn api_post(
    name: String,
    data: Json<ApiRequestData>,
    config: &State<Arc<RwLock<Config>>>,
) -> Result<(), Status> {
    let mut config = write_config(data.token.clone(), config, AuthLevel::Add)?;
    if config.urls.contains_key(&name) {
        return Err(Status::Conflict);
    }
    config.urls.insert(name, Url::from(data));
    config.write().map_err(|e| {
        eprintln!("Error writing configuration file: {}", e);
        Status::InternalServerError
    })?;
    Ok(())
}

#[put("/<name>", data = "<data>")]
fn api_put(
    name: String,
    data: Json<ApiRequestData>,
    config: &State<Arc<RwLock<Config>>>,
) -> Result<(), Status> {
    let mut config = write_config(data.token.clone(), config, AuthLevel::Modify)?;
    if !config.urls.contains_key(&name) {
        return Err(Status::NotFound);
    }
    config.urls.insert(name, Url::from(data));
    config.write().map_err(|e| {
        eprintln!("Error writing configuration file: {}", e);
        Status::InternalServerError
    })?;
    Ok(())
}

#[delete("/<name>", data = "<data>")]
fn api_delete(
    name: String,
    data: Json<ApiRequestData>,
    config: &State<Arc<RwLock<Config>>>,
) -> Result<(), Status> {
    let mut config = write_config(data.token.clone(), config, AuthLevel::Delete)?;
    if !config.urls.contains_key(&name) {
        return Err(Status::NotFound);
    }
    config.urls.remove(&name);
    config.write().map_err(|e| {
        eprintln!("Error writing configuration file: {}", e);
        Status::InternalServerError
    })?;
    Ok(())
}

fn read_config(
    token: Option<String>,
    config: &State<Arc<RwLock<Config>>>,
    authlevel: AuthLevel,
) -> Result<RwLockReadGuard<Config>, Status> {
    check_token(token, config, authlevel)?;
    Ok(config.read().map_err(|e| {
        eprintln!("Error reading configuration file: {}", e);
        Status::InternalServerError
    })?)
}

fn write_config(
    token: Option<String>,
    config: &State<Arc<RwLock<Config>>>,
    authlevel: AuthLevel,
) -> Result<RwLockWriteGuard<Config>, Status> {
    check_token(token, config, authlevel)?;
    Ok(config.write().map_err(|e| {
        eprintln!("Error writing configuration file: {}", e);
        Status::InternalServerError
    })?)
}

fn check_token(
    token: Option<String>,
    config: &State<Arc<RwLock<Config>>>,
    authlevel: AuthLevel,
) -> Result<(), Status> {
    match config
        .read()
        .map_err(|e| {
            eprintln!("Error while authenticating (read on RwLock): {}", e);
            Status::InternalServerError
        })?
        .authorized(token, authlevel)
    {
        true => Ok(()),
        false => Err(Status::Unauthorized),
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Sorry, this URL was not found."
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiRequestData {
    token: Option<String>,
    url: String,
    permanent: Option<bool>,
}

impl From<Json<ApiRequestData>> for Url {
    fn from(data: Json<ApiRequestData>) -> Self {
        Url {
            url: data.url.clone(),
            permanent: data.permanent.unwrap_or(false),
        }
    }
}
