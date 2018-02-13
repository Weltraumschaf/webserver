# Minimalistic HTTP Server

[![Build Status](https://travis-ci.org/Weltraumschaf/webserver.svg?branch=master)](https://travis-ci.org/Weltraumschaf/webserver)

Based on the multithreaded example in the [Rust Book][rust-book].

**This is not a production ready webserver!**

The goal of this project is to extend the basic implementation from the book with some useful features to learn [Rust][rust-lang].

[rust-book]:    https://doc.rust-lang.org/stable/book/second-edition/ch20-00-final-project-a-web-server.html
[rust-lang]:    https://www.rust-lang.org/

## Wanted features

- configuration options for:
    - IP to bind (done)
    - port to bind (done)
    - number of threads used in the pool (done)
    - directory where to find files to server (done)
- file based configuration
    - YAML based
    - values for 
        - IP
        - port
        - threads
        - web root
        - error doc root
        - log file path
        - host name
- logging to file
- HTTP methods (wip)
    - GET requests (done)
    - HEAD requests
    - OPTIONS request
    - error responses for unsupported methods (done)
- custom error pages
- graceful shutdown on `ctrl + c`
- basic header in the response (wip)
    - server (done)
    - accept-range/content-type w/ hard coded default (done)
    - content-type w/ right type
    