mod display;
mod user;

use std::io;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::str;

use pwhash::bcrypt;
use display::Interface;

fn main() {
    let interface = display::Terminal_Interface {};
    interface.display_menu();
    let user = interface.create_acount().expect("dsqdsq");
    println!("{:?}", user);
    let mdp_hash = bcrypt::hash(user.mdp).unwrap();
    users_store(&user.email, &mdp_hash);
    println!("mot de pass hasher : {}", mdp_hash);
}


fn users_store(id: &String, mdp: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open("users.txt").expect("Unable to open file");
    //let String id_txt = id;
    println!("Voici le pointeur : {} .", &id);
    file.write_all(id.as_bytes()).expect("Echec d'écriture");
    //file.write_all(nom<).expect("Echec d'écriture");
    Ok(())
}

