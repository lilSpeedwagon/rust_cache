use std::mem;
use std::io;

pub trait ReadFromStream {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}


impl ReadFromStream for u64 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<u64>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read u64")));
        }
        return Ok(u64::from_be_bytes(buffer));
    }
}

impl ReadFromStream for u32 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<u32>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read u32")));
        }
        return Ok(u32::from_be_bytes(buffer));
    }
}

impl ReadFromStream for u16 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<u16>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read u16")));
        }
        return Ok(u16::from_be_bytes(buffer));
    }
}


impl ReadFromStream for u8 {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        const TYPE_SIZE: usize = mem::size_of::<u8>();
        let mut buffer: [u8; TYPE_SIZE] = [0u8; TYPE_SIZE];

        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count != TYPE_SIZE {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Not enough bytes to read u8")));
        }
        return Ok(u8::from_be_bytes(buffer));
    }
}
