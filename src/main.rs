use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // Uncomment for debugging:
    // println!(
    //     "request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = 
    if buffer.starts_with(get){
       ("HTTP/1.1 200 Ok","index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read 404.html: {}", err);
        String::from("<html><body><h1>Internal Server Error</h1></body></html>")
    });
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    

    
    println!("Connection established");
}
