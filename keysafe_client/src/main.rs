extern crate core;

use display::Interface;
use state_machine::SM;

mod display;
mod state_machine;
mod user;

use bouncer::BouncerTrait;

//use Login::Login;

mod bouncer;
mod file_manager;
mod login;
mod tcp;

fn main() {
    let interface = display::TerminalInterface {};
    let mut st = SM::new(interface);
    st.start();
}
