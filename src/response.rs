use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ResponseCode {
    OK,
    NOTFOUND,
}

impl ResponseCode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ResponseCode::OK => "HTTP/1.1 200 OK\r\n",
            ResponseCode::NOTFOUND => "HTTP/1.1 404 NOT FOUND\r\n"
        }
    }
}

pub struct Response<> {
    pub status: ResponseCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl<> Response<> {

    pub fn new(status: ResponseCode, body: &Vec<u8>) -> Response<> {
        Response {
            status: status,
            body: body.to_owned(),
            headers: HashMap::new()
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(String::from(key), String::from(value));
    }
    
}

pub fn send(mut socket: &TcpStream, response: Response) {
    let mut response_string = String::from(response.status.as_str());
    
    for (key, value) in &response.headers {
        let header = format!("{}: {}\r\n", key, value);
        response_string.push_str(&header);
    }

    response_string.push_str("\r\n");
    println!("{}", response_string);

    socket.write(response_string.as_bytes()).unwrap();
    socket.write(&response.body).unwrap();
}