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

    pub fn bind(&self) {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        info!("Bind to {}", addr);
        let listener = TcpListener::bind(addr.clone())
            .expect(format!("Can't bind TCP listener on address {}!", addr).as_str());

        info!("Serving with {} threads.", self.config.threads);
        let pool = ThreadPool::new(self.config.threads);

        for stream in listener.incoming() {
            let stream = stream.expect("Cn't open TCP stream!");
            let config = self.config.clone();

            pool.execute( || {
//                Server::handle_connection(stream, config);
                Server::handle_connection_new(stream, config);
            });
        }
    }

    fn handle_connection_new(mut stream: TcpStream, config: Config) {
        let mut buffer = String::new();
        let number_of_bytes = stream.read_to_string(&mut buffer)
            .expect("Can't read from TCP stream!");
        debug!("Received {} bytes as request.", number_of_bytes);

//        let parser = RequestParser::new();
//        let request = parser.parse(buffer);
//
//        stream.write("Hello, World!".as_bytes())
//            .expect("Can't write to TCP stream!");
//        stream.flush()
//            .expect("Can't flush TCP stream!");
    }

    fn handle_connection(mut stream: TcpStream, config: Config) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, resource) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

        let filename = format!("{}/{}", config.dir(), resource);
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}