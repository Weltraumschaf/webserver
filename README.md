# Minimalistic HTTP Server

[![Build Status](https://travis-ci.org/Weltraumschaf/webserver.svg?branch=master)](https://travis-ci.org/Weltraumschaf/webserver)

Based on the multithreaded example in the [Rust Book][rust-book].

**This is not a production ready webserver!**

The goal of this project is to extend the basic implementation from the book with some useful features to learn [Rust][rust-lang].

The crate doc is [here][crate-doc].

## Build and Run

To build the webserver just run [Cargo][cargo] in the project root directory:

```bash
cargo build
```

Make your config:
```bash
cp etc/config.example.toml etc/config.toml
```

For a first test the defaults of the example config should work.

And then invoke the binary with a path to a web root directory in the project root directory:

```bash
./target/debug/webserver -c etc/config.toml
```

or run it by cargo:

```bash
cargo run -- -c etc/config.toml
```

## Wanted features

- configuration options for (done)
    - IP to bind (done)
    - port to bind (done)
    - number of threads used in the pool (done)
    - directory where to find files to server (done)
- file based configuration (wip)
    - TOML based (done)
    - values for 
        - IP  (done)
        - port  (done)
        - threads  (done)
        - web root  (done)
        - error doc root
        - log file path (done)
- logging to file (done)
- HTTP methods (done)
    - GET requests (done)
        - serve `index.html`/`index.htm` if directory requested (done)
    - HEAD requests (done)
    - OPTIONS request (done)
    - error responses for unsupported methods (done)
- custom error pages (wip)
- graceful shutdown on `ctrl + c`
- basic header in the response (done)
    - server (done)
    - accept-range/content-type w/ hard coded default (done)
    - content-type w/ right type (done)
    - date (done)
    - content-length (done)
    
[rust-book]:    https://doc.rust-lang.org/stable/book/second-edition/ch20-00-final-project-a-web-server.html
[rust-lang]:    https://www.rust-lang.org/
[cargo]:        https://doc.rust-lang.org/cargo/
[crate-doc]:    https://weltraumschaf.github.io/webserver/webserver/index.html
