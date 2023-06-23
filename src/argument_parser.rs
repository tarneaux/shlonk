/*
 * Shlonk: A simple, fast, URL shortener.
 * Author: Tarneo <tarneo@tarneo.fr>
 * License: GPL-2.0
 */

/*
 * This module parses the command-line arguments using clap.
 * The arguments are:
 * - -p <port> to specify the port to listen on (default: 8080)
 * - -h to display help
 * - -c to set the configuration file (default: /etc/shlonk/config.yml)
 */

use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    #[clap(short, long, default_value = "/etc/shlonk/config.yml")]
    pub config: String,
}
