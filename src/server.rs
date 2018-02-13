use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use Config;
use threads::ThreadPool;
use http;
use http::*;

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

            pool.execute(|| {
                Server::handle_connection_new(stream, config);
            });
        }
    }

    fn handle_connection_new(mut stream: TcpStream, config: Config) {
        let mut buffer = [0; 4096];
        stream.read(&mut buffer)
            .expect("Can't read from TCP stream!");
        let request = byte_array_to_string(buffer);
        debug!("Received data: {:?}", request);

        let request = http::parse_request(request.trim());
        debug!("Got request: {:?}", request);

        let response = build_response(config, request);

        stream.write(response.render().as_bytes())
            .expect("Can't write to TCP stream!");
        stream.flush()
            .expect("Can't flush TCP stream!");
    }
}

fn byte_array_to_string(input: [u8; 4096]) -> String {
    let mut output = String::new();

    for i in 0..input.len() {
        let ch = input[i];

        if ch == 0 {
            break;
        }

        output.push(ch as char);
    }

    output
}

fn build_response(config: Config, request: Request) -> Response {
    match request.method().as_ref() {
        "GET" => handle_get_request(config, request),
        "HEAD" => handle_head_request(config, request),
        "OPTIONS" => handle_options_request(config, request),
        _ => handle_unsupported_request(),
    }
}

fn handle_get_request(config: Config, request: Request) -> Response {
    // FIXME Do not allow directory traversal.
    // FIXME Handle dir (/) to look for index.html or index.html.
    let filename = format!("{}/{}", config.dir(), request.url());

    match File::open(filename) {
        Ok(mut f) => {
            let mut contents = String::new();
            // FIXME Handle binary data.
            f.read_to_string(&mut contents)
                .expect("Can't read resource file!");
            Response::new(
                String::from("1.1"),
                Status::Ok,
                contents)
        },
        Err(_) => {
            Response::new(
                String::from("1.1"),
                Status::NotFound,
                String::from("Not found!"))
        }
    }
}

fn handle_head_request(config: Config, request: Request) -> Response {
    // TODO Implement it.
    let response = Response::new(
        String::from("1.1"),
        Status::NotImplemented,
        String::from("Method not implemented yet!"));
    response
}

fn handle_options_request(config: Config, request: Request) -> Response {
     // TODO Implement it.
    let response = Response::new(
        String::from("1.1"),
        Status::NotImplemented,
        String::from("Method not implemented yet!"));
    response
}

fn handle_unsupported_request() -> Response {
    let mut response = Response::new(
        String::from("1.1"),
        Status::MethodNotAllowed,
        String::from("Method not supported by this HTTP server implementation!"));
    response.add_header(ResponseHeader::Allow(String::from("GET, OPTIONS, HEAD")));
    response
}
