extern crate core;




use display::Interface;
use state_machine::SM;
use crate::state_machine::State::{Logged, LogOut};


mod state_machine;
mod display;
mod user;


fn main() {
    let interface = display::Terminal_Interface {};
    let mut st = SM::new(interface);
    st.printMenu();
}

