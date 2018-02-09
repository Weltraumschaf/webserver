#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;

pub mod threads;
pub mod server;

#[derive(Debug, Clone)]
pub struct Config {
    address: String,
    port: u16,
    threads: usize,
    dir: String,
}

impl Config {
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