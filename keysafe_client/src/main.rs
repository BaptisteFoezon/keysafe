
extern crate core;


use display::Interface;
use state_machine::SM;

use crate::state_machine::State::{Logged, LogOut};

mod state_machine;
mod display;
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


mod login;
mod bouncer;
mod tcp;
mod fileManager;


fn main() {
    let interface = display::Terminal_Interface{};
    let mut st = SM::new(interface);
    st.start();
}
