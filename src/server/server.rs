use std::{net::TcpListener, net::TcpStream};
use std::io::{self, BufReader};
use std::error::Error;

use crate::server::models::{RequestReadable, RequestHeader, RequestOperation};

type Handler = fn(&mut TcpStream);


pub struct TcpServer {}

impl TcpServer {
    pub fn new() -> TcpServer {
        TcpServer {}
    }

    pub fn listen(&mut self, host: &str, port: i32) {
        let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();
        println!("Listening on {host}:{port}...");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");
            let mut buffered_reader = BufReader::new(&mut stream);

            match self.handle(&mut buffered_reader) {
                Ok(_) => println!("Ok"),
                Err(err) => {
                    println!("Error: {}", err);
                    match stream.shutdown(std::net::Shutdown::Both) {
                        Ok(_) => println!("Connection is closed on error"),
                        Err(e) => println!("Cannot close the connection: {}", e),
                    }
                }
            }
        }
    }

    fn handle(&mut self, reader: &mut dyn io::Read) -> Result<(), Box<dyn Error>> {
        let header = RequestHeader::read(reader)?;

        println!("header version={} ops-count={} body-size={}", header.proto_version, header.ops_count, header.body_size);

        for _ in 0..header.ops_count {
            let operation = RequestOperation::read(reader)?;
            println!("op-code={} body-size={}", operation.header.op_code, operation.header.body_size);
        }

        Ok(())
    }
}
