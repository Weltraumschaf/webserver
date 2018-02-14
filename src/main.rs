extern crate webserver;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

use clap::{Arg, App};
use webserver::Config;
use webserver::server::Server;

fn main() {
    simple_logger::init().unwrap();

    let matches = App::new("Weltraumschaf's Webserver")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("A minimalistic HTTP server.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Location of configuration file in TOML format.")
            .required(true))
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let config = Config::from_file(&config_file.to_string());

    if *config.port() < 1 {
        panic!("Port must be grater than 0!");
    }

    if *config.threads() < 1 {
        panic!("Number of threads must be grater than 0!");
    }

    info!("Starting web server ...");
    let server = Server::new(config);
    server.bind();
}

