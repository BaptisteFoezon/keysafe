extern crate core;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use bytes::{BufMut, BytesMut};

fn main() {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
                let answer = receive(&stream);
                if answer.eq("ask") {
                    println!("ask to enter : ");
                    send(&stream, "1");
                } else {
                    println!("> {}", answer);
                }
            };
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send(mut stream: &TcpStream, message: &str) {
    let message = format!("{}{}", message, "\n");
    println!("send > {}", message);
    stream.write_all(message.as_bytes()).expect("tcp send failed");
}

fn receive(mut stream: &TcpStream) -> String {
    //let mut data = [0u8; 200]; // using 200 byte buffer
    let mut buf = BytesMut::with_capacity(200);
    return match stream.read(&mut buf) {
        Ok(size) => {
            let c = buf; //get all the bytes
            println!("c = {:?} \n ###", &c[0..size].to_vec());
            from_utf8(&c).unwrap().to_string()
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            "".to_string()
        }
    };
}