extern crate hello;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

use clap::{Arg, App};
use hello::Config;
use hello::server::defaults::*;
use hello::server::Server;

fn main() {
    simple_logger::init().unwrap();
    info!("Starting web server ...");

    let matches = App::new("Minimalistic HTTP Server")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("A minimalistic HTTP server.")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .takes_value(true)
            .help(format!(
                "The IP address to bind to. Default is {}.",
                DEFAULT_ADDRESS).as_str()))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .help(format!(
                "The port to bind to. Default is {}.",
                DEFAULT_PORT).as_str()))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help(format!(
                "Number of parallel threads used to serve. Default is {}",
                DEFAULT_NUMBER_OF_THREADS).as_str()))
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .takes_value(true)
            .required(true)
            .help("The directory with the files to serve."))
        .get_matches();

    let address = matches.value_of("address")
        .unwrap_or(DEFAULT_ADDRESS);
    let port = matches.value_of("port")
        .unwrap_or(DEFAULT_PORT)
        .parse::<u16>()
        .unwrap();

    if port < 1 {
        panic!("Port must be grater than 0!");
    }

    let number_of_threads = matches.value_of("threads")
        .unwrap_or(DEFAULT_NUMBER_OF_THREADS)
        .parse::<usize>()
        .unwrap();

    if number_of_threads < 1 {
        panic!("Number of threads must be grater than 0!");
    }

    let dir = matches.value_of("dir").unwrap();

    let config = Config::new(address, port, number_of_threads, dir);

    let server = Server::new(config);
    server.bind();
}

