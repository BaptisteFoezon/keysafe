
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

use display::Interface;
use bouncer::Bouncer;

//use login::login;

mod display;
mod user;
mod login;
mod bouncer;



fn main() {
    let interface = display::Terminal_Interface {};
    let mut st = SM::new(interface);
    st.start();
}

