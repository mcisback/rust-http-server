use std::process;

use super::headers::HTTPHeader;

pub struct HTTPRequest {
    pub headers: Vec<HTTPHeader>,
    pub method: String,
    pub path: String,
    pub version: String,
    // body: String,
}

pub fn parse(request_str: String) -> HTTPRequest {
    let lines: Vec<&str> = request_str.lines().collect();

    let method_line: &str = lines[0];

    let parts: Vec<&str> = method_line.split(" ").collect();

    println!("METHOD LINE: {:?}", parts);

    let mut request: HTTPRequest = HTTPRequest {
        headers: Vec::new(),
        method: parts[0].to_string(),
        path: parts[1].to_string(),
        version: parts[2].to_string(),
    };

    // println!("Creating New Request: {:?}", request);

    for line in lines.iter().skip(1).map(|x| x.trim_matches('\0')) {

        println!("LINE: {}", line);

        if ! line.contains(":") {
            println!("[!] Error Malformed HTTP Headers\nMissing ':'");
            
            continue;
            // process::exit(1);
        }
        
        let mut parts: Vec<&str> = line.split(":").collect();

        if parts.len() < 2 {
            println!("[!] Error Malformed HTTP Headers\nMalformed Key: Value Header");

            process::exit(1);
        }

        let _key = parts[0].to_string();

        parts.remove(1);

        let _value = parts.join(":");

        request.headers.push(HTTPHeader {
            key: _key,
            value: _value,
        })
    }

    return request;
}