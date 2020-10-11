#[path = "./threads.rs"] mod threads;
#[path = "./request.rs"] mod request;
#[path = "./response.rs"] mod response;

use threads::ThreadPool;

use request::Request;
use request::RequestMethod;

use response::send;
use response::Response;
use response::ResponseCode;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::collections::HashMap;

pub struct SimpleServer {
    host: String,
    default_headers: HashMap<String, String>,
    pool: ThreadPool
}

impl SimpleServer {
    
    pub fn new(host: String) -> SimpleServer{
        SimpleServer {
            host: host,
            default_headers: HashMap::new(),
            pool: ThreadPool::new(10)
        }
    }

    pub fn start(&mut self) {
        self.default_headers.insert(String::from("server"), String::from("simpleserver/rs"));

        let listener = TcpListener::bind(&self.host).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(|| {
                handle_request(stream);
            });
        }
    }
}


fn handle_request(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).unwrap();

    if buffer[0] == 0 {
        return;
    }

    let request = Request::new(&buffer);
    println!("{}", request);

    let response = match request.method {
        RequestMethod::GET => process_get(request),
        _ => process_get(request)
    };

    send(&socket, response);
    socket.flush().unwrap();
}

fn get_mime_type(extension: &str) -> &'static str {
    match extension {
        "html" => "text/html",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "js" => "text/javascript",
        "json" => "application/json",
        _ => "application/octet-stream"
    }
}

fn get_file(filename: &str) -> (&'static str, Vec<u8>) {
    let file_path = format!("{}{}", "./public", filename);
    let contents = fs::read(file_path).unwrap();

    let mut parsed = filename.split('.');
    let _ = parsed.next();
    let extension = parsed.next().unwrap();
    return (get_mime_type(extension), contents);
}

fn process_get(request: Request) -> Response {
    let (mime_type, contents) = get_file(&request.location);
    let mut response = Response::new(ResponseCode::OK, &contents);

    response.add_header("accept-ranges", "bytes");
    response.add_header("access-control-allow-origin", "*");
    response.add_header("content-type", &mime_type);

    return response;
}