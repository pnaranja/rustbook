/// Chapter 20.2: Multi Threaded Web Server
use std::fs::File;
use std::io::Error;
use std::io::prelude::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(new_id: usize) -> Worker {
        Worker { id: new_id, handle: thread::spawn(|| {}) }
    }
}

struct ThreadPool {
    threads: Vec<Worker>
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // with_capacity is like Vec::new but preallocates space in the vector
        let mut workers = Vec::with_capacity(size);

        // Need to create threads
        for id in 0..size {
            workers.push(Worker::new(id));
        }


        ThreadPool { threads: workers }
    }

    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        println!("handling connection");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().into_iter()
        .map(|stream| (stream.expect("Was not able to get connection"), ThreadPool::new(3)))
        .for_each(|(stream, tpool)| tpool.execute(|| handle_connection(stream).expect("Could not read stream")))
}

/// Check request and returns 200 if "/" and 404 for anything else
fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let ok_req = |r| buffer.starts_with(r);
    let root_req = b"GET / HTTP/1.1\r\n";
    let sleep_req = b"GET /sleep HTTP/1.1\r\n";

    let resp_contents =
        if ok_req(root_req) {
            format!("HTTP/1.1 200 OK\r\n\r\n{}", read_file("hello.html")?)
        } else if ok_req(sleep_req) {
            thread::sleep(Duration::from_secs(5));
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
