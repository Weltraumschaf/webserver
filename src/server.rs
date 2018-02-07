use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use Config;
use threads::ThreadPool;

pub mod defaults {
    pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
    pub const DEFAULT_PORT: &str = "8080";
    pub const DEFAULT_NUMBER_OF_THREADS: &str = "4";
}

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server { config }
    }

    pub fn bind(self) {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        info!("Bind to {}", addr);
        let listener = TcpListener::bind(addr).unwrap();

        info!("Serving with {} threads.", self.config.threads);
        let pool = ThreadPool::new(self.config.threads);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                // FIXME
//                self.handle_connection(stream);
            });
        }
    }

    fn handle_connection(self, mut stream: TcpStream) {
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
}