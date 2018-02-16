#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;
extern crate time;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

use std::path::PathBuf;

mod file;
mod http;
mod threads;
pub mod server;

pub static APPLICATION_NAME: &'static str = "webserver";
pub static APPLICATION_DESCRIPTION: &'static str = "Weltraumschaf's Webserver";
pub static APPLICATION_VERSION: &'static str = "1.0.0";

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    address: String,
    port: u16,
    threads: usize,
    dir: String,
    log_level: String,
    log_dir: String,
}

impl Config {
    pub fn from_file(file_name: &PathBuf) -> Result<Config, &'static str> {
        let config = file::read_string(&file_name);

        match toml::from_str::<Config>(config.as_ref()) {
            // Make a copy here to invoke the constructor which validates some fields.
            Ok(config) => Config::new(
                config.address,
                config.port,
                config.threads,
                config.dir,
                config.log_level,
                config.log_dir
            ),
            Err(err) => {
                // FIXME Return appropriate error result.
                panic!("{}", err);
            },
        }
    }

    pub fn new(address: String, port: u16, threads: usize, dir: String, log_level: String, log_dir: String) -> Result<Config, &'static str> {
        if address.is_empty() {
            return Err("Config value 'address' must not be empty!");
        }

        if port < 1 {
            return Err("Config value 'port' must be grater than 0!");
        }

        if threads < 1 {
            return Err("Config value 'threads' must be grater than 0!");
        }

        if dir.is_empty() {
            return Err("Config value 'dir' must not be empty!");
        }

        // TODO Validate that it is a proper level.
        if log_level.is_empty() {
            return Err("Config value 'log_level' must not be empty!");
        }

        // TODO Validate that dir exists.
        if log_dir.is_empty() {
            return Err("Config value 'log_dir' must not be empty!");
        }

        Ok(Config { address, port, threads, dir, log_level, log_dir })
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn threads(&self) -> &usize {
        &self.threads
    }

    pub fn dir(&self) -> &String {
        &self.dir
    }

    pub fn log_level(&self) -> &String {
        &self.log_level
    }

    pub fn log_dir(&self) -> &String {
        &self.log_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn read_config_from_from_file() {
        let config = Config::from_file(&PathBuf::from("test/config.toml"))
            .expect("Can't read config fixture file!");

        assert_eq!(config.address(), "127.0.0.1");
        let expected_port: u16 = 8080;
        assert_eq!(config.port(), &expected_port);
        let expected_threads: usize = 4;
        assert_eq!(config.threads(), &expected_threads);
        assert_eq!(config.dir(), "web_root/");
        assert_eq!(config.log_level(), "debug");
        assert_eq!(config.log_dir(), "logs/");
    }

    #[test]
    fn new_validates_address_not_empty() {
        let config = Config::new(
            String::from(""),
            8080,
            4,
            String::from("dir"),
            String::from("debug"),
            String::from("logs/"));

        assert_that!(config, is(equal_to(Err("Config value 'address' must not be empty!"))));
    }

    #[test]
    fn new_validates_port_not_less_than_one() {
        let config = Config::new(
            String::from("127.0.0.1"),
            0,
            4,
            String::from("dir"),
            String::from("debug"),
            String::from("logs/"));

        assert_that!(config, is(equal_to(Err("Config value 'port' must be grater than 0!"))));
    }

    #[test]
    fn new_validates_threads_not_less_than_one() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            0,
            String::from("dir"),
            String::from("debug"),
            String::from("logs/"));

        assert_that!(config, is(equal_to(Err("Config value 'threads' must be grater than 0!"))));
    }

    #[test]
    fn new_validates_dir_not_empty() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            4,
            String::from(""),
            String::from("debug"),
            String::from("logs/"));

        assert_that!(config, is(equal_to(Err("Config value 'dir' must not be empty!"))));
    }

    #[test]
    fn new_validates_log_level_not_empty() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            4,
            String::from("web_root/"),
            String::from(""),
            String::from("logs/"));

        assert_that!(config, is(equal_to(Err("Config value 'log_level' must not be empty!"))));
    }

    #[test]
    fn new_validates_log_dir_not_empty() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            4,
            String::from("web_root/"),
            String::from("debug"),
            String::from(""));

        assert_that!(config, is(equal_to(Err("Config value 'log_dir' must not be empty!"))));
    }
}