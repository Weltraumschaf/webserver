use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::net::TcpListener;
use std::net::TcpStream;
use time;
use Config;
use file;
use threads::ThreadPool;
use http;
use http::{Request, Response, ResponseHeader, Status};

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server { config }
    }

    pub fn bind(&self) -> Result<(), &'static str> {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        info!("Bind to {}", addr);

        let listener = if let Ok(listener) = TcpListener::bind(addr.clone()) {
            listener
        } else {
            return Err("Can't bind TCP listener on address!");
        };

        info!("Serving with {} threads.", self.config.threads);
        let pool = ThreadPool::new(self.config.threads);
        format!("Listening on http://{}:{}/", self.config.address, self.config.port);

        for stream in listener.incoming() {
            let stream = stream.expect("Cn't open TCP stream!");
            let config = self.config.clone();

            pool.execute(|| {
                Server::handle_connection_new(stream, config);
            });
        }

        Ok(())
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
    let mut wanted_resource = create_resource_path(config.dir(), request.url());
    debug!("Wanted resource is {:?}", wanted_resource);

    if wanted_resource.is_dir() {
        let mut wanted_resource_file = wanted_resource.join("index.html");
        debug!("Wanted resource is a directory. Looking for {:?}", wanted_resource_file);

        if !wanted_resource_file.exists() {
            wanted_resource_file = wanted_resource.join("index.htm");
            debug!("Wanted resource is a directory. Looking for {:?}", wanted_resource_file);
        }

        if !wanted_resource_file.exists() {
            debug!("Nothing appropriate found!");
            return not_found_response();
        } else {
            wanted_resource = wanted_resource_file;
        }
    }

    debug!("Requested resource {:?}", wanted_resource);

    let mut response = if wanted_resource.exists() {
        let mut contents = file::read_bytes(&wanted_resource);
        let mut response = Response::new(
            String::from("1.1"),
            Status::Ok,
            contents);
        response.add_header(
            ResponseHeader::ContentType(
                format!("{}; charset=utf-8", determine_content_type(&wanted_resource))));
        response
    } else {
        not_found_response()
    };

    let content_length = response.content_length();
    response.add_header(ResponseHeader::ContentLength(content_length));
    response.add_header(ResponseHeader::Date(formatted_now()));
    response.add_header(ResponseHeader::Server(String::from("Weltraumschaf's Webserver")));
    response.add_header(ResponseHeader::AcceptRanges(String::from("none")));
    response
}

fn not_found_response() -> Response {
    let mut response = Response::new(
        String::from("1.1"),
        Status::NotFound,
        "Not found!".as_bytes().to_vec());
    response.add_header(ResponseHeader::ContentType(String::from("text/plain; charset=utf-8")));
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

fn determine_content_type(file_name: &PathBuf) -> String {
    match file_name.extension() {
        Some(extension) => {
            match extension.to_str().unwrap() {
                "html" | "htm" => String::from("text/html"),
                "css" => String::from("text/css"),
                "js" => String::from("text/javascript"),
                "ico" => String::from("image/x-icon"),
                _ => String::from("text/plain"),
            }
        },
        None => String::from("text/plain"),
    }
}

fn formatted_now() -> String {
    // http://man7.org/linux/man-pages/man3/strftime.3.html
    // Wed, 14 Feb 2018 12:17:24 GMT
    time::strftime("%a, %d %b %Y %H:%M:%S %Z", &time::now())
        .expect("Can't format date!")
}

fn create_resource_path(web_root: &String, resource_url: &String) -> PathBuf {
    let relative_resource_url = relativize_uri(resource_url);
    Path::new(web_root).join(relative_resource_url)
}

fn relativize_uri(resource_url: &String) -> String {
    if resource_url.starts_with("/") {
        resource_url[1..].to_string()
    } else {
        resource_url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn test_determine_content_type_from_file_name() {
        assert_that!(
            determine_content_type(&PathBuf::from("")),
            is(equal_to(String::from("text/plain")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("index.html")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("new.index.htm")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("/foo/bar/new.index.html")),
            is(equal_to(String::from("text/html")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("foo.abr.css")),
            is(equal_to(String::from("text/css")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("/foo/bar/new.index.js")),
            is(equal_to(String::from("text/javascript")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("/foo/bar/new.index.js")),
            is(equal_to(String::from("text/javascript")))
        );
        assert_that!(
            determine_content_type(&PathBuf::from("/foo/bar/favicon.ico")),
            is(equal_to(String::from("image/x-icon")))
        );
    }

    #[test]
    fn test_relativize_uri() {
        assert_that!(relativize_uri( & String::from("foo/bar/bax.html")),
            is(equal_to(String::from("foo/bar/bax.html"))));
        assert_that!(relativize_uri( & String::from("/foo/bar/bax.html")),
            is(equal_to(String::from("foo/bar/bax.html"))));
    }

    #[test]
    fn test_create_resource_path() {
        assert_that!(
            create_resource_path( & String::from("web_root/"), & String::from("/")),
            is(equal_to(PathBuf::from("web_root/"))));
        assert_that!(
            create_resource_path( & String::from("web_root/"), &String::from("/index.html")),
            is(equal_to(PathBuf::from("web_root/index.html"))));
        assert_that!(
            create_resource_path( & String::from("web_root/"), & String::from("/css/main.css")),
            is(equal_to(PathBuf::from("web_root/css/main.css"))));
    }
}