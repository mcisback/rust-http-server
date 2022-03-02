use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
// use std::time::Duration;

use std::path::Path;
use std::env;

mod headers;
mod request;
mod response;

// const SERVER_DIR: &str = ;
const DEFAULT_HTML_FILE: &str = "/index.html";

pub fn run(host: &str, port: &str) {
    let listener = TcpListener::bind(
        format!("{}:{}", host, port)
    ).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // let request = HTTPRequest::new();
    
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..]).to_string();

    println!("\n\nRequest: \n{}\n", request_str);

    // let lines: Vec<&str> = request_str.lines().collect();

    // println!("LINES: {:?}", lines);


    let request: request::HTTPRequest = request::parse(request_str);

    // println!("Created Request: {:?}", request);
    println!(
        "\nPARSED REQUEST:\nMETHOD: {}\nPATH: {}\nHTTP_VERSION: {}\n\n",
        request.method,
        request.path,
        request.version
    );

    let mut path = request.path.as_str();
    let mut http_response_status = 200;
    let mut http_response_message = "OK";
    
    if path.eq("/") {
        path = DEFAULT_HTML_FILE;
    }

    if !Path::new( &get_server_file_path( path ) ).exists() {
        path = "/404.html";
        http_response_status = 404;
        http_response_message = "Not Found";    
    }

    println!("Final Request Path: {}", get_server_file_path( path ));

    let contents = fs::read_to_string(
        get_server_file_path( path )
    ).unwrap();

    let mut response: response::HTTPResponse = response::HTTPResponse {
        status: http_response_status,
        message: http_response_message.to_string(),
        version: "HTTP/1.1".to_string(),
        body: contents.to_string(),
        stream: stream,
    };

    response.send_to_client();
}

fn get_server_file_path(filename: &str) -> String {
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    return String::from(format!("{}/server-data/{}", current_dir, filename));
}
