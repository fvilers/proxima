use proxima::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // TODO: use configured address and port instead of 127.0.0.1:8080
    // TODO: gracefully handle error instead of unwrap()
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // TODO: use configured thread pool size instead of 4
    // TODO: gracefully handle error instead of unwrap()
    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        // TODO: gracefully handle error instead of unwrap()
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // TODO: handle requests of size over 1024 bytes
    let mut buffer = [0; 1024];

    // TODO: gracefully handle error instead of unwrap()
    stream.read(&mut buffer).unwrap();

    let contents = "Hello world!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // TODO: gracefully handle error instead of unwrap()
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
