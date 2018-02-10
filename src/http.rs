#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    method: String,
    url: String,
    version: String,
    host: String,
    user_agent: String,
    accept: String,
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

    fn finish(&self) -> Request {
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

pub struct RequestParser {}

impl RequestParser {
    pub fn new() -> RequestParser {
        RequestParser {}
    }

    pub fn parse(&self, input: String) -> Request {
        let mut builder = RequestBuilder::new();
        let mut lexer = RequestLexer::new(input);

        while lexer.has_next() {
            match lexer.next() {
                RequestToken::Method(method) => {
                    builder.with_method(&method);
                },
                RequestToken::Url(url) => {
                    builder.with_url(&url);
                },
                RequestToken::Version(version) => {
                    builder.with_version(&version);
                },
                RequestToken::HeaderName(name) => {},
                RequestToken::HeaderValue(value) => {},
                RequestToken::EOL => {
                    continue; // ignore EOL
                },
                RequestToken::EOT => {
                    break; // end of token stream
                },
                _ => {
                    panic!("Unrecognized token!");
                },
            }
        }

        builder.finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum RequestToken {
    Method(String),
    Url(String),
    Version(String),
    HeaderName(String),
    HeaderValue(String),
    EOL,
    EOT,
}

#[derive(Debug)]
struct RequestLexer {
    input: String,
    index: usize,
}

const END_OF_INPUT: char = 0 as char;

impl RequestLexer {
    fn new(input: String) -> RequestLexer {
        RequestLexer {
            input: input.clone(),
            index: 0,
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.input.len()
    }

    fn next(&mut self) -> RequestToken {
        let mut is_in_first_line = true;

        loop {
            let mut ch = self.current_char();

            match ch {
                END_OF_INPUT => return RequestToken::EOT,
                '\r' => {
                    self.next_char(); // consume the carriage return
                    ch = self.current_char();

                    match ch {
                        '\n' => {
                            is_in_first_line = false;
                            return RequestToken::EOL
                        },
                        _ => panic!("Expected \\n after \\r, but saw '{}'!", ch),
                    }
                },
                _ => {
                    let mut buffer = String::new();

                    if is_in_first_line {
                        loop {
                            if ch.is_whitespace() {
                                self.next_char(); // ignore white space
                                break;
                            }

                            if ch == '\r' || ch == '\n' {
                                break; // leave the eol chars for the outer llop
                            }

                            self.next_char();
                            buffer.push(ch);
                        }

                        if self.is_http_method(&buffer) {
                            return RequestToken::Method(buffer);
                        } else if self.is_http_version(&buffer) {
                            return RequestToken::Version(buffer);
                        } else {
                            return RequestToken::Url(buffer);
                        }
                    } else {
                        if ch.is_whitespace() {
                            self.next_char(); // ignore white space
                            continue;
                        }

                        if ch == '\r' || ch == '\n' {
                            return RequestToken::HeaderValue(buffer);
                        }

                        if ch == ':' {
                            self.next_char(); // consume colon
                            return RequestToken::HeaderName(buffer);
                        }
                    }
                },
            }
        }
    }

    fn current_char(&self) -> char {
        match self.input.chars().nth(self.index) {
            Some(ch) => ch,
            None => 0 as char
        }
    }

    fn next_char(&mut self) {
        self.index += 1;
    }

    fn is_http_method(&self, input: &str) -> bool {
        match input {
            "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH" => true,
            _ => false,
        }
    }

    fn is_http_version(&self, input: &str) -> bool {
        if input.starts_with("HTTP/") {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn lex_request() {
        let request_fixture = "GET /foo HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: curl/7.54.0\r\n
Accept: */*\r\n";

        let mut sut = RequestLexer::new(request_fixture.to_string());

        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::Method(String::from("GET")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::Url(String::from("/foo")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::Version(String::from("HTTP/1.1")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::EOL)));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderName(String::from("Host")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderValue(String::from("localhost:8080")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::EOL)));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderName(String::from("User-Agent")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderValue(String::from("curl/7.54.0")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::EOL)));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderName(String::from("Accept")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::HeaderValue(String::from("*/*")))));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::EOL)));
        assert_that!(sut.has_next(), is(true));
        assert_that!(sut.next(), is(equal_to(RequestToken::EOT)));
        assert_that!(sut.has_next(), is(false));
    }

    #[test]
    #[ignore]
    fn parse_request() {
        let request_fixture = "GET /foo HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: curl/7.54.0\r\n
Accept: */*\r\n";

        let sut = RequestParser::new();
        let request = sut.parse(&request_fixture.to_string());

        assert_that!(
        request, is(equal_to(
            Request {
                method: String::from("GET"),
                url: String::from("/foo"),
                version: String::from("HTTP/1.1"),
                host: String::from("localhost:8080"),
                user_agent: String::from("curl/7.54.0"),
                accept: String::from("*/*"),
            }
        )));
    }
}