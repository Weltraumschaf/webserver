#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;

pub mod threads;
pub mod server;

pub struct Config<'a> {
    pub address: &'a str,
    pub port: u16,
    pub threads: usize,
    pub dir: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(address: &'a str, port: u16, threads: usize, dir: &'a str) -> Config<'a> {
        Config { address, port, threads, dir }
    }
}