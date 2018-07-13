use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().into_iter()
        .for_each(|stream |{
            handle_connection(stream.expect("Was not able to get connection"));
    })

}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Could not read buffer");

    println!("Request: \n{}", String::from_utf8_lossy(&buffer[..]));
}
