use kafka_protocol::{ApiKey, ApiVersion, Error};
use std::convert::TryFrom;
use std::io::Cursor;
use async_std::net::TcpStream;
use async_std::io;
use async_std::prelude::*;
use async_std::task;
use byteorder::{BigEndian, ReadBytesExt};

fn main() -> io::Result<()> {
    task::block_on(async {
        let mut stream = TcpStream::connect(("127.0.0.1", 9094)).await?;
        println!("Connected to {}", &stream.peer_addr()?);

        let bytes = [
            0x00, 0x00, 0x00, 0x13,       // Length (19)
            0x00, 0x12,                   // API Key: ApiVersions (18)
            0x00, 0x00,                   // API Version (0)
            0x00, 0x00, 0x00, 0x01,       // Correlation ID (1)
            0x00, 0x05,                   // String length (5)
            b'f', b'r', b'a', b'n', b'z', // Client ID (franz)
            0x00, 0x00, 0x00, 0x00        // ?????
        ];

        stream.write_all(&bytes).await?;
        stream.flush().await?;

        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        let len = i32::from_be_bytes(buf);

        let mut buf = vec![0u8; len as usize];
        stream.read_exact(&mut buf).await?;

        let mut cursor = Cursor::new(buf);

        let correlation_id = cursor.read_i32::<BigEndian>()?;
        let error = cursor.read_i16::<BigEndian>()?;
        let array_size = cursor.read_i32::<BigEndian>()?;

        println!("Size: {:?}", len);
        println!("Correlation ID: {:?}", correlation_id);
        println!("Error: {:?} ({})", Error::try_from(error).unwrap(), error);
        println!("Supported API Versions:");

        let mut versions = Vec::new();

        for _ in 0..array_size {
            let key = cursor.read_i16::<BigEndian>()?;
            let min = cursor.read_i16::<BigEndian>()?;
            let max = cursor.read_i16::<BigEndian>()?;

            versions.push(ApiVersion::new(ApiKey::try_from(key).unwrap(), min, max))
        }

        for version in &versions {
            println!("    {:?}", version);
        }

        Ok(())
    })
}
