extern crate core;

use bouncer::BouncerTrait;
use display::Interface;
use state_machine::SM;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use tcp::{MyTcp, MyTcpTrait};

mod display;
mod state_machine;
mod user;

//use Login::Login;

mod bouncer;
mod file_manager;
mod login;
mod tcp;


fn handle_client(mut stream: TcpStream) {
    let myTcp = MyTcp::new(&stream);
    let interface = display::TcpInterface {
        myTcp
    };
    let mut st = SM::new(interface);
    st.start();
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    println!("new user is connected");
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
/*
fn main() {
    let interface = display::TerminalInterface {};
    let mut st = SM::new(interface);
    st.start();
}
*/