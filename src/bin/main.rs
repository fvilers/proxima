use proxima::Opt;
use proxima::ThreadPool;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use structopt::StructOpt;

fn main() {
    // TODO: merge options with configuration file
    let options = Opt::from_args();

    // TODO: gracefully handle error instead of unwrap()
    let ip_addr = Ipv4Addr::from_str(&options.address).unwrap();
    let socket = SocketAddr::new(IpAddr::V4(ip_addr), options.port);

    // TODO: gracefully handle error instead of unwrap()
    let listener = TcpListener::bind(socket).unwrap();

    // TODO: gracefully handle error instead of unwrap()
    let pool = ThreadPool::new(options.thread_pool_size).unwrap();

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
