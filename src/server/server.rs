use std::{io::Read, net::TcpListener, net::TcpStream};
use std::io::{self, BufReader};
use std::error::Error;

use crate::server::serialize;

type Handler = fn(&mut TcpStream);


struct RequestHeader {
    proto_version: i8,
    reserved1: i8,
    ops_count: i16,
    checksum: i32,
    body_size: i32,
    reserved2: i32,
}

pub struct TcpServer {
    
}

impl TcpServer {

    pub fn new() -> TcpServer {
        return TcpServer{};
    }

    pub fn listen(&mut self, host: &str, port: i32) {
        let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();
        println!("Listening on {host}:{port}...");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");
            let mut buffered_reader = BufReader::new(&mut stream);

            match self.handle(&mut buffered_reader) {
                Ok(_) => { println!("Ok") },
                Err(err) => {
                    println!("Error: {}", err);
                    match stream.shutdown(std::net::Shutdown::Both) {
                        Ok(_) => println!("connection is closed on error"),
                        Err(e) => println!("cannot close the connection: {}", e)
                    }
                }
            }

        }
    }

    fn handle(&mut self, reader: &mut dyn io::Read) -> Result<(), Box<dyn Error>> {
        // Parse and check the header.
        let header = self.read_header(reader)?;

        println!("{}-{}-{}-{}", header.proto_version, header.ops_count, header.checksum, header.body_size);

        // Parse commands.

        // Route each command.

        // Prepare response and return.
        Ok(())
    }

    fn read_header(&mut self, reader: &mut dyn io::Read) -> Result<RequestHeader, Box<dyn Error>> {
        const HEADER_SIZE: usize = std::mem::size_of::<RequestHeader>();
        let mut header_buf: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
        let size = reader.read(&mut header_buf)?;
        if size != HEADER_SIZE {
            println!("header size is {}. expected {}", size, HEADER_SIZE);
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Invalid request header of size {size}"))));
        }

        let version: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved1: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let ops_count: i16 = serialize::ReadFromStream::from_stream(reader)?;
        let checksum: i32 = serialize::ReadFromStream::from_stream(reader)?;
        let body_size: i32 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved2: i32 = serialize::ReadFromStream::from_stream(reader)?;
        
        if reserved1 != 0 || reserved2 != 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Reserved header parts are expected to be zeroes."))));
        }

        return Ok(RequestHeader{
            proto_version: version,
            reserved1: reserved1,
            ops_count: ops_count,
            checksum: checksum,
            body_size: body_size,
            reserved2: reserved2,
        });
    }

}
