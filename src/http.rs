use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    method: String,
    url: String,
    version: String,
    host: String,
    user_agent: String,
    accept: String,
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
pub struct Response {
    version: String,
    status: Status,
    body: String,
}

impl Response {
    pub fn new(version: String, status: Status, body: String) -> Response {
        Response { version, status, body }
    }

    pub fn render(&self) -> String {
        format!("HTTP/{} {}\r\n\r\n{}", self.version, self.status, self.body)
    }
}

#[derive(Debug)]
pub enum ResponseHeader {
    // Allow: GET, POST, HEAD
    Allow(String),
}

#[derive(Debug)]
pub enum Status {
    Ok,
    // Client errors 4xx - 499
    NotFound,
    MethodNotAllowed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 NOT FOUND",
            Status::MethodNotAllowed => "405 METHOD NOT ALLOWED",
        };
        write!(f, "{}", printable)
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
                        _ => panic!("Unexpected header name '{}'!", name),
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
                }
            ))
        );
    }

    #[test]
    fn test_render_response() {
        let sut = Response::new(
            String::from("1.1"),
            Status::Ok,
            String::from("Hello. World!")
        );

        assert_that!(
            sut.render(),
            is(equal_to(
                String::from("HTTP/1.1 200 OK\r\n\r\nHello. World!")
            ))
        );
    }

    #[test]
    fn status_fmt() {
        assert_that!(format!("{}", Status::Ok).as_str(), is(equal_to("200 OK")));
        assert_that!(format!("{}", Status::NotFound).as_str(), is(equal_to("404 NOT FOUND")));
        assert_that!(format!("{}", Status::MethodNotAllowed).as_str(), is(equal_to("405 METHOD NOT ALLOWED")));
    }
}