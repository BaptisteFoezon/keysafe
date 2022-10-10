extern crate core;

use display::Interface;
use state_machine::SM;

use crate::state_machine::State::{LogOut, Logged};

mod display;
mod state_machine;
mod user;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str;

use pwhash::bcrypt;

use bouncer::Bouncer;

//use login::login;

mod bouncer;
mod fileManager;
mod login;
mod tcp;

fn main() {
    let interface = display::Terminal_Interface {};
    let mut st = SM::new(interface);
    st.start();
}
