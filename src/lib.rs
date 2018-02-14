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

mod file;
mod http;
mod threads;
pub mod server;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    address: String,
    port: u16,
    threads: usize,
    dir: String,
}

impl Config {
    pub fn from_file(file_name: &String) -> Config {
        let config = file::read_string(&file_name);
        toml::from_str(config.as_ref())
            .expect(format!("Can't parse config {}!", file_name).as_ref())
    }

    pub fn new(address: String, port: u16, threads: usize, dir: String) -> Config {
        Config { address, port, threads, dir }
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
    use file;

    #[test]
    fn read_config_from_from_file() {
        let config = Config::from_file(&String::from("test/config.toml"));

        assert_eq!(config.address(), "127.0.0.1");
        let expected_port: u16 = 8080;
        assert_eq!(config.port(), &expected_port);
        let expected_threads: usize = 4;
        assert_eq!(config.threads(), &expected_threads);
        assert_eq!(config.dir(), "web_root/");
    }
}