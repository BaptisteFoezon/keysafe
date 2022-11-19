extern crate core;

use display::TcpInterfaceTrait;
use state_machine::SM;
use state_machine::StateMachineTrait;
use std::net::{TcpListener, TcpStream};
use std::thread;
use tcp::{MyTcp, MyTcpTrait};

mod state_machine;
mod display;
mod bouncer;
mod login;
mod tcp;
mod file_manager;
mod user;
mod config;

fn handle_client(mut stream: TcpStream) {
    let myTcp = MyTcp::new(stream);
    let interface = display::TcpInterface::new(myTcp);
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

