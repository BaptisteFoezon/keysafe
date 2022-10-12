use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) trait MyTcpTrait {
    fn new(stream: TcpStream) -> MyTcp;
    fn read(&mut self) -> io::Result<()>;
    fn write(&mut self, message: &str) -> io::Result<()>;
}

#[derive(Debug)]
pub(crate) struct MyTcp {
    pub(crate) stream: TcpStream,
}

impl MyTcpTrait for MyTcp {
    fn new(stream: TcpStream) -> MyTcp {
        MyTcp { stream }
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
