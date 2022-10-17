extern crate core;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    println!("Bienvenue dans keysafe");
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
                send(&stream, "hello");
                let answer = receive(&stream);
                println!("{}", answer);
                if answer.eq("menu") {
                    println!("j'affiche le menu")
                }
                else {
                    println!("continue .. ")
                }
            };
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send(mut stream: &TcpStream, message: &str) {
    dbg!("send some data ...");
    stream.write(message.as_ref()).unwrap();
}

fn receive(mut stream: &TcpStream) -> &str {
    let mut c = "";
    let mut data = [0 as u8; 6]; // using 6 byte buffer
    match stream.read_exact(&mut data) {
        Ok(_) => {
             let c = from_utf8(&data).unwrap();
            println!("{}", c)
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    return  c;
}