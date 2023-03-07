use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connections(stream);
    }
}

fn handle_connections(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let status_line: &str = "HTTP/1.1 200 OK";
        let page_path: &str = "html/index.html";
        send_response(status_line, page_path, &stream);
    } else {
        let status_line: &str = "HTTP/1.1 404 NOT FOUND";
        let page_path: &str = "html/404.html";
        send_response(status_line, page_path, &stream);
    }
}

fn send_response(status_line: &str, html_path: &str, mut stream: &TcpStream) {
    let content: String = fs::read_to_string(html_path).unwrap();

    let response: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
