use std::mem;
use std::io;

pub trait ReadFromStream {
    fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}


macro_rules! impl_read_from_stream {
    ($($t:ty),*) => {
        $(
            impl ReadFromStream for $t {
                fn from_stream(stream: &mut dyn io::Read) -> Result<Self, Box<dyn std::error::Error>> {
                    const TYPE_SIZE: usize = mem::size_of::<$t>();
                    let mut buffer = [0u8; TYPE_SIZE];

                    let bytes_count = stream.read(&mut buffer)?;
                    if bytes_count != TYPE_SIZE {
                        return Err(Box::new(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Not enough bytes to read {}", std::any::type_name::<$t>()),
                        )));
                    }

                    Ok(<$t>::from_be_bytes(buffer))
                }
            }
        )*
    };
}

impl_read_from_stream!(u8, u16, u32, u64);
