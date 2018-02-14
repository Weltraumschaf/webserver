use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use time;
use Config;
use file;
use threads::ThreadPool;
use http;
use http::*;

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

        stream.write(&response.render())
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
    let file_name = format!("{}/{}", config.dir(), request.url());
    debug!("Try to serve file {}.", file_name);

    let mut response = if file::exists(&file_name) {
        let mut contents = file::read_bytes(&file_name);
        let mut response = Response::new(
            String::from("1.1"),
            Status::Ok,
            contents);
        response.add_header(
            ResponseHeader::ContentType(
                format!("{}; charset=utf-8", determine_content_type(&file_name))));
        response
    } else {
        let mut response = Response::new(
            String::from("1.1"),
            Status::NotFound,
            "Not found!".as_bytes().to_vec());
        response.add_header(ResponseHeader::ContentType(String::from("text/plain; charset=utf-8")));
        response
    };

    let content_length = response.content_length();
    response.add_header(ResponseHeader::ContentLength(content_length));
    response.add_header(ResponseHeader::Date(formatted_now()));
    response.add_header(ResponseHeader::Server(String::from("Weltraumschaf's Webserver")));
    response.add_header(ResponseHeader::AcceptRanges(String::from("none")));
    response
}

fn handle_head_request(config: Config, request: Request) -> Response {
    // TODO Implement it.
    let response = Response::new(
        String::from("1.1"),
        Status::NotImplemented,
        "Method not implemented yet!".as_bytes().to_vec());
    response
}

fn handle_options_request(config: Config, request: Request) -> Response {
    // TODO Implement it.
    let response = Response::new(
        String::from("1.1"),
        Status::NotImplemented,
        "Method not implemented yet!".as_bytes().to_vec());
    response
}

fn handle_unsupported_request() -> Response {
    let mut response = Response::new(
        String::from("1.1"),
        Status::MethodNotAllowed,
        "Method not supported by this HTTP server implementation!".as_bytes().to_vec());
    response.add_header(ResponseHeader::Allow(String::from("GET, OPTIONS, HEAD")));
    response
}

fn determine_content_type(file_name: &String) -> String {
    match extract_file_extension(&file_name).as_ref() {
        "html" | "htm" => String::from("text/html"),
        "css" => String::from("text/css"),
        "js" => String::from("text/javascript"),
        "ico" => String::from("image/x-icon"),
        _ => String::from("text/plain"),
    }
}

fn extract_file_extension(file_name: &String) -> String {
    match file_name.rfind(".") {
        Some(pos) => file_name[pos + 1..].to_string(),
        None => String::from(""),
    }
}

fn formatted_now() -> String {
    // http://man7.org/linux/man-pages/man3/strftime.3.html
    // Wed, 14 Feb 2018 12:17:24 GMT
    time::strftime("%a, %d %b %Y %H:%M:%S %Z", &time::now())
        .expect("Can't format date!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn test_determine_content_type_from_file_name() {
        assert_that!(
            determine_content_type(&String::from("")),
            is(equal_to(String::from("text/plain")))
        );
        assert_that!(
            determine_content_type(&String::from("index.html")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&String::from("new.index.htm")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&String::from("/foo/bar/new.index.html")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&String::from("foo.abr.css")),
            is(equal_to(String::from("text/css")))
        );
        assert_that!(
            determine_content_type(&String::from("/foo/bar/new.index.js")),
            is(equal_to(String::from("text/javascript")))
        );
        assert_that!(
            determine_content_type(&String::from("/foo/bar/new.index.js")),
            is(equal_to(String::from("text/javascript")))
        );
        assert_that!(
            determine_content_type(&String::from("/foo/bar/favicon.ico")),
            is(equal_to(String::from("image/x-icon")))
        );
    }

    #[test]
    fn test_extract_file_extension() {
        assert_that!(
            extract_file_extension(&String::from("")),
            is(equal_to(String::from("")))
        );
        assert_that!(
            extract_file_extension(&String::from("index.html")),
            is(equal_to(String::from("html")))
        );
        assert_that!(
            extract_file_extension(&String::from("new.index.htm")),
            is(equal_to(String::from("htm")))
        );
        assert_that!(
            extract_file_extension(&String::from("/foo/bar/new.index.html")),
            is(equal_to(String::from("html")))
        );
        assert_that!(
            extract_file_extension(&String::from("foo.abr.css")),
            is(equal_to(String::from("css")))
        );
        assert_that!(
            extract_file_extension(&String::from("/foo/bar/new.index.js")),
            is(equal_to(String::from("js")))
        );
    }
}