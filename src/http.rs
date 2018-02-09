const EOL: &str = "\r\n";

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    method: String,
    url: String,
    version: String,
    host: String,
    user_agent: String,
    accept: String,
}

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

    fn with_method(mut self, new_method: &String) -> RequestBuilder {
        self.method = new_method.clone();
        self
    }

    fn with_url(mut self, new_url: &String) -> RequestBuilder {
        self.url = new_url.clone();
        self
    }

    fn with_version(mut self, new_version: &String) -> RequestBuilder {
        self.version = new_version.clone();
        self
    }

    fn with_host(mut self, new_host: &String) -> RequestBuilder {
        self.host = new_host.clone();
        self
    }

    fn with_user_agent(mut self, new_user_agent: &String) -> RequestBuilder {
        self.user_agent = new_user_agent.clone();
        self
    }

    fn with_accept(mut self, new_accept: &String) -> RequestBuilder {
        self.accept = new_accept.clone();
        self
    }
}

pub struct RequestParser {}

impl RequestParser {
    pub fn new() -> RequestParser {
        RequestParser {}
    }

    pub fn parse(&self, input: &String) -> Request {
        for char in input.chars() {
            print!("{}", char);
        }

        let builder = RequestBuilder::new();
        builder.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn it_works() {
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