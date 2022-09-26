mod display;
use std::io;
use std::fs;
use pwhash::bcrypt;
use display::Interface;
use crate::display::{user, User};

fn main() {
    let interface = display::Terminal_Interface{};
   interface.display_menu();
    let user = interface.create_acount();
    let mdp_hash = bcrypt::hash(user.mdp).unwrap();
    println!("mot de pass hasher : {}", mdp_hash );

    if let choice= String::from("2") {
        println!("choix numéro 1");
    }

}

