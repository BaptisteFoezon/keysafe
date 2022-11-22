use bytes::{BufMut, BytesMut};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub trait MyTcpTrait {
    fn new(stream: TcpStream) -> MyTcp;
    fn ask(&mut self) -> String;
    fn read(&mut self) -> String;
    fn send(&mut self, message: String);
}

pub struct MyTcp {
    stream: TcpStream,
}

impl MyTcpTrait for MyTcp {
    fn new(stream: TcpStream) -> MyTcp {
        MyTcp { stream }
    }
    fn read(&mut self) -> String {
        let mut data = [0u8; 200]; // using 100 byte buffer
        return match self.stream.read(&mut data) {
            Ok(_) => {
                let c = from_utf8(&data).unwrap(); //get all the bytes
                c.to_string()
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
                "".to_string()
            }
        };
    }

    fn ask(&mut self) -> String {
        let mes = String::from("ask");
        self.send(mes);
        self.read()
    }

    fn send(&mut self, message: String) {
        println!("send message > {}", message);
        let bytes_written = self.stream.write(message.as_bytes()).expect("tcp send failed");
        println!("Sent bytes > {} ", bytes_written);
        if bytes_written < message.len() {
            println!("Sent {}/{} bytes", bytes_written, message.len());
        }
        self.stream.flush().expect("tcp flush failed");
    }
}
