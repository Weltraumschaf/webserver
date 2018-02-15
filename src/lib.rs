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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    address: String,
    port: u16,
    threads: usize,
    dir: String,
}

impl Config {
    pub fn from_file(file_name: &PathBuf) -> Result<Config, &'static str> {
        let config = file::read_string(&file_name);

        match toml::from_str::<Config>(config.as_ref()) {
            Ok(config) => Config::new(config.address, config.port, config.threads, config.dir),
            Err(_) => Err("Can't parse config!"),
        }
    }

    pub fn new(address: String, port: u16, threads: usize, dir: String) -> Result<Config, &'static str> {
        if address.is_empty() {
            return Err("Address must not be empty!");
        }

        if port < 1 {
            return Err("Port must be grater than 0!");
        }

        if threads < 1 {
            return Err("Number of threads must be grater than 0!");
        }

        if dir.is_empty() {
            return Err("Dir must not be empty!");
        }

        Ok(Config { address, port, threads, dir })
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
    }

    #[test]
    fn new_validates_address_not_empty() {
        let config = Config::new(
            String::from(""),
            8080,
            4,
            String::from("dir"));

        assert_that!(config, is(equal_to(Err("Address must not be empty!"))));
    }

    #[test]
    fn new_validates_port_not_less_than_one() {
        let config = Config::new(
            String::from("127.0.0.1"),
            0,
            4,
            String::from("dir"));

        assert_that!(config, is(equal_to(Err("Port must be grater than 0!"))));
    }

    #[test]
    fn new_validates_threads_not_less_than_one() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            0,
            String::from("dir"));

        assert_that!(config, is(equal_to(Err("Number of threads must be grater than 0!"))));
    }

    #[test]
    fn new_validates_dir_not_empty() {
        let config = Config::new(
            String::from("127.0.0.1"),
            8080,
            4,
            String::from(""));

        assert_that!(config, is(equal_to(Err("Dir must not be empty!"))));
    }
}