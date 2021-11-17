extern crate dotenv;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use dotenv::dotenv;
use proxima::Opt;
use proxima::ThreadPool;
use std::env;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, Clone)]
struct ApplicationError<'a> {
    msg: &'a str,
}

// General comment: Rust supports async-await / Futures now,
// this could be an interesting iteration to migrate it.

// You could add automated build / test on the repo with a GitHub action.
// Also, don't hesitate to run 'cargo clippy' for static analysis of your code,
// and some usefull recommandation.
// To apply standard formatting: 'cargo fmt -all'

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Proxima starting...");

    ctrlc::set_handler(move || {
        debug!("Exit signal received");
        info!("Proxima exiting...");
        process::exit(0);
    })
    .expect("Error setting exit signal handler");

    process::exit(match app() {
        Ok(_) => {
            info!("Proxima exiting...");
            0
        }
        Err(err) => {
            error!("ApplicationError: {:?}", err.msg);
            1
        }
    });
}

fn app<'a>() -> Result<(), ApplicationError<'a>> {
    // TODO: merge options with configuration file
    let options = Opt::from_args();
    let address = env::var("ADDRESS").unwrap_or(options.address);
    let ip_addr = Ipv4Addr::from_str(&address).map_err(|_| ApplicationError {
        msg: "Invalid IP Address",
    })?;
    let port = match env::var("PORT") {
        Ok(p) => u16::from_str(&p).unwrap_or(options.port),
        Err(_) => options.port,
    };
    let socket = SocketAddr::new(IpAddr::V4(ip_addr), port);
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
                error!("Connection failed: {}", e);
                continue;
            }
        };

        pool.execute(|| match handle_connection(stream) {
            Ok(()) => (),
            Err(e) => error!("Error handling connection: {}", e),
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    // TODO: handle requests of size over 1024 bytes
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let contents = "Hello world!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
