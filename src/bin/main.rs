use proxima::Opt;
use proxima::ThreadPool;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, Clone)]
struct ApplicationError<'a> {
    msg: &'a str,
}

fn main() {
    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("ApplicationError: {:?}", err.msg);
            1
        }
    });
}

fn app<'a>() -> Result<(), ApplicationError<'a>> {
    // TODO: merge options with configuration file
    let options = Opt::from_args();
    let ip_addr = Ipv4Addr::from_str(&options.address).map_err(|_| ApplicationError {
        msg: "Invalid IP Address",
    })?;
    let socket = SocketAddr::new(IpAddr::V4(ip_addr), options.port);
    let listener = TcpListener::bind(socket).map_err(|_| ApplicationError {
        msg: "Unable to bind to socket",
    })?;
    let pool = ThreadPool::new(options.thread_pool_size).map_err(|_| ApplicationError {
        msg: "Unable to initialize the thread pool",
    })?;

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                continue;
            }
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    // TODO: handle requests of size over 1024 bytes
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to read stream: {}", e);
            return;
        }
    }

    let contents = "Hello world!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    match stream.write(response.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write to stream: {}", e);
            return;
        }
    }

    match stream.flush() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to flush stream: {}", e);
            return;
        }
    }
}
