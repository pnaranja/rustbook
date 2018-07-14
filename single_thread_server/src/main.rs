use std::fs::File;
use std::io::Error;
use std::io::prelude::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let resp_contents = read_file("hello.html").expect("Could not read html file");

    listener.incoming().into_iter()
        .map(|stream| handle_connection(stream.expect("Was not able to get connection"))
            .expect("Could not read stream"))
        .for_each(|stream|
            return_response(stream, resp_contents.clone()).expect("Could not write to stream")
        );
}

fn handle_connection(mut stream: TcpStream) -> Result<TcpStream, Error> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request_str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: \n{}", request_str);

    Ok(stream)
}

fn read_file(file_loc: &str) -> Result<String, Error> {
    let mut file = File::open(file_loc)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn return_response(mut stream: TcpStream, resp: String) -> Result<(), Error> {
    let full_resp = format!("HTTP/1.1 200 OK\r\n\r\n{}", resp);
    stream.write(full_resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}