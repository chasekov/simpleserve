use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum RequestMethod {
    GET,
    POST
}

pub struct RequestHeader {
    pub key: String,
    pub value: String
}

impl fmt::Display for RequestHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request Header: key:{}, value:{}", self.key, self.value)
    }
}

pub struct Request {
    pub method: RequestMethod,
    pub location: String,
    pub version: String,
    pub headers: HashMap<String, String>
}

impl Request {

    pub fn new(input: &[u8]) -> Request {
        let newline_char: u8 = '\n' as u8;
        let mut headers = input.split(|val| val == &newline_char);

        let (method, location, version) = parse_type(headers.next().unwrap());
        let mut request_headers: HashMap<String, String> = HashMap::new();

        while let Some(header) = headers.next() {
            if String::from_utf8_lossy(header).contains(":") {
                let (key, value) = parse_header(header);
                request_headers.insert(key, value);
            }
        }

        Request {
            method: method,
            location: location,
            version: version,
            headers: request_headers
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Request {} {}", self.location, self.version);
        Ok(())
    }
}

fn parse_header(input: &[u8]) -> (String, String){
    let colon_char: u8 = ':' as u8;
    let mut args = input.split(|val| val == &colon_char);
    let key = String::from_utf8_lossy(args.next().unwrap());
    let value = String::from_utf8_lossy(args.next().unwrap());

    (
        key.to_string(), 
        value.to_string()
    )
}

fn parse_type(input: &[u8]) -> (RequestMethod, String, String) {
    let space_char: u8 = ' ' as u8;
    let mut args = input.split(|val| val == &space_char);

    let method_text = String::from_utf8_lossy(args.next().unwrap());
    let method = match &method_text[..] {
        "GET" => RequestMethod::GET,
        "POST" => RequestMethod::POST,
        _ => RequestMethod::GET,
    };

    let location = String::from_utf8_lossy(args.next().unwrap());
    let version = String::from_utf8_lossy(args.next().unwrap());
    assert!(args.next().is_none());

    (
        method, 
        location.to_string(),
        version.to_string()
    )
}
