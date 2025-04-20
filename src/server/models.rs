use std::error::Error;
use std::io;

use crate::server::serialize;

pub trait RequestReadable {
    fn read(reader: &mut dyn io::Read) -> Result<(Self), Box<dyn Error>> where Self: Sized;
}

// A single server request header.
// Reserved fields are added for padding and future compatibility.
// They are expected to be zeroes.
pub struct RequestHeader {
    pub proto_version: u8,
    reserved1: u8,
    pub ops_count: u16,
    checksum: u32,
    pub body_size: u32,
    reserved2: u32,
}

impl RequestReadable for RequestHeader {
    fn read(reader: &mut dyn io::Read) -> Result<Self, Box<dyn Error>> {
        let version: u8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved1: u8 = serialize::ReadFromStream::from_stream(reader)?;
        let ops_count: u16 = serialize::ReadFromStream::from_stream(reader)?;
        let checksum: u32 = serialize::ReadFromStream::from_stream(reader)?;
        let body_size: u32 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved2: u32 = serialize::ReadFromStream::from_stream(reader)?;
    
        if reserved1 != 0 || reserved2 != 0 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Reserved header parts are expected to be zeroes. Got {reserved1} and {reserved2}"
                ),
            )));
        }
    
        Ok(Self {
            proto_version: version,
            reserved1,
            ops_count,
            checksum,
            body_size,
            reserved2,
        })
    }
}


// A single operation header.
// Reserved fields are added for padding and future compatibility.
// They are expected to be zeroes.
// The body size is the size of the operation body, not including the header.
pub struct RequestOperationHeader {
    pub op_code: u8,
    reserved1: u8,
    reserved2: u16,
    pub body_size: u32,
}

impl RequestReadable for RequestOperationHeader {
    fn read(reader: &mut dyn io::Read) -> Result<Self, Box<dyn Error>> {
        let op_code: u8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved1: u8 = serialize::ReadFromStream::from_stream(reader)?;
        let reserved2: u16 = serialize::ReadFromStream::from_stream(reader)?;
        let body_size: u32 = serialize::ReadFromStream::from_stream(reader)?;
    
        if reserved1 != 0 || reserved2 != 0 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Reserved header parts are expected to be zeroes. Got {reserved1} and {reserved2}"
                ),
            )));
        }
    
        Ok(RequestOperationHeader {
            op_code,
            reserved1,
            reserved2,
            body_size,
        })
    }
}


// A single operation in a request.
pub struct RequestOperation {
    pub header: RequestOperationHeader,
    pub body: Vec<u8>,
}

impl RequestReadable for RequestOperation {
    fn read(reader: &mut dyn io::Read) -> Result<Self, Box<dyn Error>> {
        let header = RequestOperationHeader::read(reader)?;
    
        let mut body_buf = vec![0; header.body_size as usize];
        let size = reader.read(&mut body_buf)?;
        if size != header.body_size as usize {
            println!("operation body size is {}. expected {}", size, size);
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid request operation of size {size}")
            )));
        }
    
        Ok(RequestOperation {
            header,
            body: body_buf,
        })
    }
}
