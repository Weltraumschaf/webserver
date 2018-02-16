extern crate webserver;
extern crate flexi_logger;
#[macro_use]
extern crate log;

use flexi_logger::{Logger, opt_format};

extern crate clap;

use std::process;
use clap::{Arg, App};
use std::path::{PathBuf};
use flexi_logger::FlexiLoggerError;
use webserver::Config;
use webserver::server::Server;

fn main() {
    let config = read_config().unwrap_or_else(|err| {
        println!("Problem reading config file: {}", err);
        process::exit(1);
    });

    let _ = setup_logging(&config).unwrap_or_else(|err| {
        println!("Logger initialization failed with {}", err);
        process::exit(2);
    });

    info!("Starting web server ...");
    let server = Server::new(config);
    server.bind().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(3);
    });
}

fn read_config() -> Result<Config, &'static str> {
    let matches = App::new(webserver::APPLICATION_DESCRIPTION)
        .version(webserver::APPLICATION_VERSION)
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("A minimalistic HTTP server.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Location of configuration file in TOML format.")
            .required(true))
        .get_matches();

    let config_file = matches.value_of("config").expect("No config file given!");
    let config_file = PathBuf::from(config_file);
    Config::from_file(&config_file)
}

fn setup_logging(config: &Config) -> Result<(), FlexiLoggerError> {
    let level = config.log_level().clone();
    let log_config = format!(
        "warn, {}={}, {}={}",
        webserver::APPLICATION_NAME, level,
        webserver::APPLICATION_NAME, level);
    println!("Use log config: {}", log_config);
    Logger::with_str(log_config.as_str())
        .log_to_file()
        .print_message()
        .directory(config.log_dir().clone())
        .duplicate_error()
        .format(opt_format)
        .start()
}
