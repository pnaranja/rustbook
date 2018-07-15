/// Chapter 20.1: Single Threaded Web Server
use std::fs::File;
use std::io::Error;
use std::io::prelude::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().into_iter()
        .map(|stream| stream.expect("Was not able to get connection"))
        .for_each(|stream| handle_connection(stream).expect("Could not read stream"));
}

/// Check request and returns 200 if "/" and 404 for anything else
fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let resp_contents =
        if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            format!("HTTP/1.1 200 OK\r\n\r\n{}", read_file("hello.html")?)
        } else {
            format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", read_file("404.html")?)
        };

    return_response(stream, resp_contents)
}

/// Read file and return contents as a String
fn read_file(file_loc: &str) -> Result<String, Error> {
    let mut file = File::open(file_loc)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Return response
fn return_response(mut stream: TcpStream, resp: String) -> Result<(), Error> {
    stream.write(resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}