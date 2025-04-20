use std::{net::TcpListener, net::TcpStream};
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

struct RequestOperationHeader {
    op_code: i8,
    reserved1: i8,
    reserved2: i16,
    body_size: i32,
}

struct RequestOperation {
    header: RequestOperationHeader,
    body: Vec<u8>,
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
        for _ in 0..header.ops_count {
            let operation = self.read_operation(reader)?;
            println!("{}-{}", operation.header.op_code, operation.header.body_size);
        }

        // Route each command.

        // Prepare response and return.
        Ok(())
    }

    fn read_header(&mut self, reader: &mut dyn io::Read) -> Result<RequestHeader, Box<dyn Error>> {
        let version: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved1: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let ops_count: i16 = serialize::ReadFromStream::from_stream(reader)?;
        let checksum: i32 = serialize::ReadFromStream::from_stream(reader)?;
        let body_size: i32 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved2: i32 = serialize::ReadFromStream::from_stream(reader)?;
        
        if reserved1 != 0 || reserved2 != 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Reserved header parts are expected to be zeroes. Got {reserved1} and {reserved2}"))));
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

    fn read_operation_header(&mut self, reader: &mut dyn io::Read) -> Result<RequestOperationHeader, Box<dyn Error>> {
        let op_code: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved1: i8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved2: i16 = serialize::ReadFromStream::from_stream(reader)?;
        let body_size: i32 = serialize::ReadFromStream::from_stream(reader)?;
        
        if reserved1 != 0 || reserved2 != 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Reserved header parts are expected to be zeroes. Got {reserved1} and {reserved2}"))));
        }

        return Ok(RequestOperationHeader{
            op_code: op_code,
            reserved1: reserved1,
            reserved2: reserved2,
            body_size: body_size,
        });
    }

    fn read_operation(&mut self, reader: &mut dyn io::Read) -> Result<RequestOperation, Box<dyn Error>> {
        let header = self.read_operation_header(reader)?;

        let mut body_buf = vec![0; header.body_size as usize];
        let size = reader.read(&mut body_buf)?;
        if size != header.body_size as usize {
            println!("operation body size is {}. expected {}", size, size);
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Invalid request operation of size {size}"))));
        }

        return Ok(RequestOperation{
            header: header,
            body: body_buf,
        });
    }
}
