# Minimalistic HTTP Server

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
    - directory where to find files to server (wip)
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
- HTTP methods
    - GET requests
    - HEAD requests
    - OPTIONS request
    - error responses for unsupported methods
- custom error pages
- graceful shutdown on `ctrl + c`
- basic header in the response
    - ...
    