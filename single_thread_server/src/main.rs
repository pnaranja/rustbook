use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::Read;
use std::io::Error;
use std::io::Write;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().into_iter()
        .for_each(|stream | {
            let a = handle_connection(stream.expect("Was not able to get connection"))
                .expect("Could not read stream");
            return_response(a).expect("Could not write to stream");
        });

}

fn handle_connection(mut stream: TcpStream) -> Result<TcpStream, Error> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request_str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: \n{}", request_str);

    Ok(stream)
}

fn return_response(mut stream: TcpStream) -> Result<(), Error>{
    let resp = "HTTP/1.1 200 OK\r\n\r\nHello!";
    stream.write(resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}