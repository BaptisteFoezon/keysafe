use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) trait myTcp {
    fn new(stream: TcpStream) -> mytcp;
    fn read(&mut self) -> io::Result<()>;
    fn write(&mut self, message: &str) -> io::Result<()>;
}

#[derive(Debug)]
pub(crate) struct mytcp {
    pub(crate) stream: TcpStream,
}

impl myTcp for mytcp {
    fn new(stream: TcpStream) -> mytcp {
        mytcp { stream }
    }
    fn read(&mut self) -> io::Result<()> {
        let mut rx_bytes = [0u8; 10];
        // Read from the current data in the TcpStream
        self.stream.read(&mut rx_bytes)?;

        let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
        eprintln!("{}", received);
        Ok(())
    }

    fn write(&mut self, message: &str) -> io::Result<()> {
        self.stream.write(message.as_bytes()).unwrap();
        Ok(())
    }
}
