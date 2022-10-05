extern crate core;




use display::Interface;
use stateMachine::SM;
use crate::stateMachine::State::{Logged, LogOut};


mod stateMachine;
mod display;
mod user;


fn main() {
    let interface = display::Terminal_Interface {};
    let mut st = SM::new(interface);
    st.printMenu();
}

