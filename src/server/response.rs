use std::net::TcpStream;
use std::io::Write;

pub struct HTTPResponse {
    pub status: i32,
    pub message: String,
    pub version: String,
    pub body: String,
    pub stream: TcpStream,
}

impl HTTPResponse {
    pub fn send_to_client(&mut self) {
        let response = format!(
            "{} {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.version,
            self.status,
            self.message,
            self.body.len(),
            self.body
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}