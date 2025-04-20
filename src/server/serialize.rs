use std::mem;
use std::io;

pub trait ReadFromStream {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}


impl ReadFromStream for i64 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<i64>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read i64")));
        }
        return Ok(i64::from_be_bytes(buffer));
    }
}

impl ReadFromStream for i32 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<i32>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read i32")));
        }
        return Ok(i32::from_be_bytes(buffer));
    }
}

impl ReadFromStream for i16 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<i16>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read i16")));
        }
        return Ok(i16::from_be_bytes(buffer));
    }
}


impl ReadFromStream for i8 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<i8>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read i8")));
        }
        return Ok(i8::from_be_bytes(buffer));
    }
}
