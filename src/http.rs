use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    method: String,
    url: String,
    version: String,
    host: String,
    user_agent: String,
    accept: String,
    upgrade_insecure_requests: String,
    accept_language: String,
    accept_encoding: String,
    cookie: String,
    connection: String,
    referer: String,
    cache_control: String,
}

impl Request {
    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[derive(Debug)]
struct RequestBuilder {
    method: String,
    url: String,
    version: String,
    host: String,
    user_agent: String,
    accept: String,
    upgrade_insecure_requests: String,
    accept_language: String,
    accept_encoding: String,
    cookie: String,
    connection: String,
    referer: String,
    cache_control: String,
}

impl RequestBuilder {
    fn new() -> RequestBuilder {
        RequestBuilder {
            method: String::from(""),
            url: String::from(""),
            version: String::from(""),
            host: String::from(""),
            user_agent: String::from(""),
            accept: String::from(""),
            upgrade_insecure_requests: String::from(""),
            accept_language: String::from(""),
            accept_encoding: String::from(""),
            cookie: String::from(""),
            connection: String::from(""),
            referer: String::from(""),
            cache_control: String::from(""),
        }
    }

    fn create(&self) -> Request {
        Request {
            method: self.method.clone(),
            url: self.url.clone(),
            version: self.version.clone(),
            host: self.host.clone(),
            user_agent: self.user_agent.clone(),
            accept: self.accept.clone(),
            upgrade_insecure_requests: self.upgrade_insecure_requests.clone(),
            accept_language: self.accept_language.clone(),
            accept_encoding: self.accept_encoding.clone(),
            cookie: self.cookie.clone(),
            connection: self.connection.clone(),
            referer: self.referer.clone(),
            cache_control: self.cache_control.clone(),
        }
    }

    fn with_method(&mut self, new_method: &String) {
        self.method = new_method.clone();
    }

    fn with_url(&mut self, new_url: &String) {
        self.url = new_url.clone();
    }

    fn with_version(&mut self, new_version: &String) {
        self.version = new_version.clone();
    }

    fn with_host(&mut self, new_host: &String) {
        self.host = new_host.clone();
    }

    fn with_user_agent(&mut self, new_user_agent: &String) {
        self.user_agent = new_user_agent.clone();
    }

    fn with_accept(&mut self, new_accept: &String) {
        self.accept = new_accept.clone();
    }

    fn with_accept_language(&mut self, new_accept_language: &String) {
        self.accept_language = new_accept_language.clone();
    }

    fn with_accept_encoding(&mut self, new_accept_encoding: &String) {
        self.accept_encoding = new_accept_encoding.clone();
    }

    fn with_cookie(&mut self, new_cookie: &String) {
        self.cookie = new_cookie.clone();
    }

    fn with_connection(&mut self, new_connection: &String) {
        self.connection = new_connection.clone();
    }

    fn with_upgrade_insecure_requests(&mut self, new_upgrade_insecure_requests: &String) {
        self.upgrade_insecure_requests = new_upgrade_insecure_requests.clone();
    }

    fn with_referer(&mut self, new_referer: &String) {
        self.referer = new_referer.clone();
    }

    fn with_cache_control(&mut self, new_cache_control: &String) {
        self.cache_control = new_cache_control.clone();
    }
}

#[derive(Debug)]
pub struct Response {
    version: String,
    status: Status,
    headers: Vec<ResponseHeader>,
    body: String,
}

impl Response {
    pub fn new(version: String, status: Status, body: String) -> Response {
        Response { version, status, headers: Vec::new(), body }
    }

    pub fn render(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(format!("HTTP/{} {}\r\n", self.version, self.status).as_str());

        for header in self.headers.iter() {
            buffer.push_str(format!("{}\r\n", header).as_str());
        }

        buffer.push_str("\r\n");
        buffer.push_str(self.body.as_str());
        buffer
    }

    pub fn add_header(&mut self, header: ResponseHeader) {
        self.headers.push(header);
    }
}

// https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html
#[derive(Debug)]
pub enum ResponseHeader {
    // Allow: GET, POST, HEAD
    Allow(String),
    Server(String),
    // Accept-Ranges: none
    AcceptRanges(String),
    // Content-Type: text/html; charset=utf-8
    ContentType(String),
}

impl fmt::Display for ResponseHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ResponseHeader::Allow(ref value) => format!("Allow: {}", value),
            ResponseHeader::Server(ref value) => format!("Server: {}", value),
            ResponseHeader::AcceptRanges(ref value) => format!("Accept-Ranges: {}", value),
            ResponseHeader::ContentType(ref value) => format!("Content-Type: {}", value),
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
pub enum Status {
    Ok,
    // Client errors 400 - 499
    NotFound,
    MethodNotAllowed,
    // Server errors 500 - 599
    InternalServerError,
    NotImplemented,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 NOT FOUND",
            Status::MethodNotAllowed => "405 METHOD NOT ALLOWED",
            Status::InternalServerError => "500 INTERNAL SERVER ERROR",
            Status::NotImplemented => "501 NOT IMPLEMENTED",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum RequestToken {
    Method(String),
    Url(String),
    Version(String),
    HeaderName(String),
    HeaderValue(String),
    EndOfText,
}

fn split_lines(input: &str) -> Vec<&str> {
    input.trim().split("\r\n").collect::<Vec<&str>>()
}

fn parse_first_line(line: &str) -> (RequestToken, RequestToken, RequestToken) {
    let parts: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let method = parts[0].trim();
    let url = parts[1].trim();
    let full_version = parts[2].trim();
    let version = &full_version[5..];

    (RequestToken::Method(method.to_string()),
        RequestToken::Url(url.to_string()),
        RequestToken::Version(version.to_string()))
}

fn parse_non_first_line(line: &str) -> (RequestToken, RequestToken) {
    let colon_position = line.find(":")
        .expect("No colon found in line!");
    let header_name = line[0..colon_position].trim();
    let header_value = line[colon_position + 1..].trim();

    (RequestToken::HeaderName(header_name.to_string()),
        RequestToken::HeaderValue(header_value.to_string()))
}

fn scan_request(request: &str) -> Vec<RequestToken> {
    let lines = split_lines(request);
    let mut tokens: Vec<RequestToken> = Vec::new();
    let mut is_first_line = true;

    for line in lines {
        if is_first_line {
            let (method, uri, version) = parse_first_line(line);
            tokens.push(method);
            tokens.push(uri);
            tokens.push(version);
            is_first_line = false;
        } else {
            let (name, value) = parse_non_first_line(line);
            tokens.push(name);
            tokens.push(value);
        }
    };

    tokens.push(RequestToken::EndOfText);
    tokens
}

pub fn parse_request(request: &str) -> Request {
    if request.is_empty() {
        panic!("Empty request input!");
    }

    let mut builder = RequestBuilder::new();
    let tokens = scan_request(request);
    let mut tokens_iterator = tokens.iter();

    loop {
        let token = tokens_iterator.next()
            .expect("No more tokens_iterator, but expected more!");

        match token {
            &RequestToken::Method(ref method) => builder.with_method(&method),
            &RequestToken::Url(ref url) => builder.with_url(&url),
            &RequestToken::Version(ref version) => builder.with_version(&version),
            &RequestToken::HeaderName(ref name) => {
                let value_token = tokens_iterator.next()
                    .expect(format!("Expecting a value for header '{}'!", &name).as_str());

                if let &RequestToken::HeaderValue(ref value) = value_token {
                    match name.as_str() {
                        "Host" => builder.with_host(&value.clone()),
                        "User-Agent" => builder.with_user_agent(&value.clone()),
                        "Accept" => builder.with_accept(&value.clone()),
                        "Accept-Language" => builder.with_accept_language(&value.clone()),
                        "Accept-Encoding" => builder.with_accept_encoding(&value.clone()),
                        "Cookie" => builder.with_cookie(&value.clone()),
                        "Connection" => builder.with_connection(&value.clone()),
                        "Upgrade-Insecure-Requests" => builder.with_upgrade_insecure_requests(&value.clone()),
                        "Referer" => builder.with_referer(&value.clone()),
                        "Cache-Control" => builder.with_cache_control(&value.clone()),
                        _ => debug!("Unexpected header name '{}'!", name),
                    }
                }
            },
            &RequestToken::EndOfText => break,
            _ => panic!("Should not happen!"),
        }
    }

    builder.create()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn test_split_lines() {
        let request_fixture = "GET /foo HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n";

        assert_that(
            split_lines(request_fixture),
            is(equal_to(
                vec!(
                    "GET /foo HTTP/1.1",
                    "Host: localhost:8080",
                    "User-Agent: curl/7.54.0",
                    "Accept: */*")
            ))
        );
    }

    #[test]
    fn test_parse_first_line() {
        let first_line_fixture = "GET /foo HTTP/1.1";

        assert_that!(
            parse_first_line(first_line_fixture),
            is(equal_to(
                (
                    RequestToken::Method(String::from("GET")),
                    RequestToken::Url(String::from("/foo")),
                    RequestToken::Version(String::from("1.1"))
                )
            ))
        )
    }

    #[test]
    fn test_parse_non_first_line_host_header() {
        let host_header_fixture = "Host: localhost:8080";

        assert_that!(
            parse_non_first_line(host_header_fixture),
            is(equal_to(
                (
                    RequestToken::HeaderName(String::from("Host")),
                    RequestToken::HeaderValue(String::from("localhost:8080"))
                )
            ))
        );
    }

    #[test]
    fn test_parse_non_first_line_user_agent_header() {
        let user_agent_header_fixture = "User-Agent: curl/7.54.0";

        assert_that!(
            parse_non_first_line(user_agent_header_fixture),
            is(equal_to(
                (
                    RequestToken::HeaderName(String::from("User-Agent")),
                    RequestToken::HeaderValue(String::from("curl/7.54.0"))
                )
            ))
        );
    }

    #[test]
    fn test_parse_non_first_line_accept_header() {
        let accept_header_fixture = "Accept: */*";

        assert_that!(
            parse_non_first_line(accept_header_fixture),
            is(equal_to(
                (
                    RequestToken::HeaderName(String::from("Accept")),
                    RequestToken::HeaderValue(String::from("*/*"))
                )
            ))
        );
    }

    #[test]
    fn test_scan_request() {
        let request_fixture = "GET /foo HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n";

        assert_that!(
            scan_request(request_fixture),
            is(equal_to(
                vec!(
                    RequestToken::Method(String::from("GET")),
                    RequestToken::Url(String::from("/foo")),
                    RequestToken::Version(String::from("1.1")),
                    RequestToken::HeaderName(String::from("Host")),
                    RequestToken::HeaderValue(String::from("localhost:8080")),
                    RequestToken::HeaderName(String::from("User-Agent")),
                    RequestToken::HeaderValue(String::from("curl/7.54.0")),
                    RequestToken::HeaderName(String::from("Accept")),
                    RequestToken::HeaderValue(String::from("*/*")),
                    RequestToken::EndOfText
                )
            ))
        )
    }

    #[test]
    fn test_parse_request() {
        let request_fixture = "GET /foo HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n";

        assert_that!(
            parse_request(request_fixture),
            is(equal_to(
                Request {
                    method: String::from("GET"),
                    url: String::from("/foo"),
                    version: String::from("1.1"),
                    host: String::from("localhost:8080"),
                    user_agent: String::from("curl/7.54.0"),
                    accept: String::from("*/*"),
                    upgrade_insecure_requests: String::from(""),
                    accept_language: String::from(""),
                    accept_encoding: String::from(""),
                    cookie: String::from(""),
                    connection: String::from(""),
                    referer: String::from(""),
                    cache_control: String::from(""),
                }
            ))
        );
    }

    #[test]
    fn test_parse_request_firefox() {
        let request_fixture = "GET /hello.html HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:58.0) Gecko/20100101 Firefox/58.0\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\nAccept-Language: en,en-US;q=0.7,de;q=0.3\r\nAccept-Encoding: gzip, deflate\r\nReferer: http://localhost:8080/index.html\r\nCookie: _ga=GA1.1.822344465.1506073564; JSESSIONID=node0ag061949mqugevd0gpoadofu2.node0; teamscale-session-8080=admin:A9H8KhGk7eCIm4TR_TJqLKPiJ8Vgm9yQ; io=ngTvRgwb_vVkB9ckAAAP; NXSESSIONID=7f81c463-4a5e-4bc4-9040-8b9779ce9f41; token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdGF0dXMiOiJzdWNjZXNzIiwiZGF0YSI6eyJpZCI6MSwiZW1haWwiOiJhZG1pbkBqdWljZS1zaC5vcCIsInBhc3N3b3JkIjoiMDE5MjAyM2E3YmJkNzMyNTA1MTZmMDY5ZGYxOGI1MDAiLCJjcmVhdGVkQXQiOiIyMDE3LTEwLTIyIDEzOjAxOjIyLjAwMCArMDA6MDAiLCJ1cGRhdGVkQXQiOiIyMDE3LTEwLTIyIDEzOjAxOjIyLjAwMCArMDA6MDAifSwiaWF0IjoxNTA4Njc3OTM4LCJleHAiOjE1MDg2OTU5Mzh9.YJkvadkpWXgx6IpjdrXRv8MurV8Tlms1npl2yqa8pm8\r\nConnection: keep-alive\r\nUpgrade-Insecure-Requests: 1\r\nCache-Control: max-age=0\r\n\r\n";

        assert_that!(
            parse_request(request_fixture),
            is(equal_to(
                Request {
                    method: String::from("GET"),
                    url: String::from("/hello.html"),
                    version: String::from("1.1"),
                    host: String::from("localhost:8080"),
                    user_agent: String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:58.0) Gecko/20100101 Firefox/58.0"),
                    accept: String::from("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                    upgrade_insecure_requests: String::from("1"),
                    accept_language: String::from("en,en-US;q=0.7,de;q=0.3"),
                    accept_encoding: String::from("gzip, deflate"),
                    cookie: String::from("_ga=GA1.1.822344465.1506073564; JSESSIONID=node0ag061949mqugevd0gpoadofu2.node0; teamscale-session-8080=admin:A9H8KhGk7eCIm4TR_TJqLKPiJ8Vgm9yQ; io=ngTvRgwb_vVkB9ckAAAP; NXSESSIONID=7f81c463-4a5e-4bc4-9040-8b9779ce9f41; token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdGF0dXMiOiJzdWNjZXNzIiwiZGF0YSI6eyJpZCI6MSwiZW1haWwiOiJhZG1pbkBqdWljZS1zaC5vcCIsInBhc3N3b3JkIjoiMDE5MjAyM2E3YmJkNzMyNTA1MTZmMDY5ZGYxOGI1MDAiLCJjcmVhdGVkQXQiOiIyMDE3LTEwLTIyIDEzOjAxOjIyLjAwMCArMDA6MDAiLCJ1cGRhdGVkQXQiOiIyMDE3LTEwLTIyIDEzOjAxOjIyLjAwMCArMDA6MDAifSwiaWF0IjoxNTA4Njc3OTM4LCJleHAiOjE1MDg2OTU5Mzh9.YJkvadkpWXgx6IpjdrXRv8MurV8Tlms1npl2yqa8pm8"),
                    connection: String::from("keep-alive"),
                    referer: String::from("http://localhost:8080/index.html"),
                    cache_control: String::from("max-age=0"),
                }
            ))
        );
    }

    #[test]
    fn test_render_response_without_headers() {
        let sut = Response::new(
            String::from("1.1"),
            Status::Ok,
            String::from("Hello, World!")
        );

        assert_that!(
            sut.render(),
            is(equal_to(
                String::from("HTTP/1.1 200 OK\r\n\r\nHello, World!")
            ))
        );
    }

    #[test]
    fn test_render_response_with_headers() {
        let mut sut = Response::new(
            String::from("1.1"),
            Status::MethodNotAllowed,
            String::from("This is not allowed!")
        );

        sut.add_header(ResponseHeader::Allow(String::from("GET, POST, HEAD")));

        assert_that!(
            sut.render(),
            is(equal_to(
                String::from("HTTP/1.1 405 METHOD NOT ALLOWED\r\nAllow: GET, POST, HEAD\r\n\r\nThis is not allowed!")
            ))
        );
    }

    #[test]
    fn status_fmt() {
        assert_that!(
            format!("{}", Status::Ok).as_str(),
            is(equal_to("200 OK")));
        assert_that!(
            format!("{}", Status::NotFound).as_str(),
            is(equal_to("404 NOT FOUND")));
        assert_that!(
            format!("{}", Status::MethodNotAllowed).as_str(),
            is(equal_to("405 METHOD NOT ALLOWED")));
        assert_that!(
            format!("{}", Status::InternalServerError).as_str(),
            is(equal_to("500 INTERNAL SERVER ERROR")));
        assert_that!(
            format!("{}", Status::NotImplemented).as_str(),
            is(equal_to("501 NOT IMPLEMENTED")));
    }

    #[test]
    fn response_header_fmt() {
        assert_that!(
            format!("{}", ResponseHeader::Allow(String::from("GET, POST, HEAD"))).as_str(),
            is(equal_to("Allow: GET, POST, HEAD")));
    }
}