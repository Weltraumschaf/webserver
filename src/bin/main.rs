extern crate hello;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use clap::{Arg, App};
use hello::Config;
use hello::threads::ThreadPool;
use hello::server;

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
                server::DEFAULT_ADDRESS).as_str()))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .help(format!(
                "The port to bind to. Default is {}.",
                server::DEFAULT_PORT).as_str()))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help(format!(
                "Number of parallel threads used to serve. Default is {}",
                server::DEFAULT_NUMBER_OF_THREADS).as_str()))
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .takes_value(true)
            .required(true)
            .help("The directory with the files to serve."))
        .get_matches();

    let address = matches.value_of("address")
        .unwrap_or(server::DEFAULT_ADDRESS);
    let port = matches.value_of("port")
        .unwrap_or(server::DEFAULT_PORT)
        .parse::<u16>()
        .unwrap();

    if port < 1 {
        panic!("Port must be grater than 0!");
    }

    let number_of_threads = matches.value_of("threads")
        .unwrap_or(server::DEFAULT_NUMBER_OF_THREADS)
        .parse::<usize>()
        .unwrap();

    if number_of_threads < 1 {
        panic!("Number of threads must be grater than 0!");
    }

    let dir = matches.value_of("dir").unwrap();

    let config = Config::new(address, port, number_of_threads, dir);

    bind_listener(config);
}

fn bind_listener(config: Config) {
    let addr = format!("{}:{}", config.address, config.port);
    info!("Bind to {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    info!("Serving with {} threads.", config.threads);
    let pool = ThreadPool::new(config.threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
