use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str;

use pwhash::bcrypt;

use display::Interface;
use bouncer::Bouncer;

mod display;
mod user;
mod login;
mod bouncer;


fn main() {
    let interface = display::Terminal_Interface {};
    let mut choice = interface.display_menu().expect(" ");
    println!("{}", choice);
    if choice.eq("1") {
        println!("Creation d'un compte ...");
        let user = interface.create_account().expect("dsqdsq");
        println!("{:?}", user);
        let mdp_hash = bcrypt::hash(user.mdp).unwrap();
        users_store(&user.pseudo, &mdp_hash).expect("Une erreur est survenue");
        println!("mot de pass hasher : {}", mdp_hash);
        interface.display_create_account_success();
    }
    if choice.eq("2") {

        let mybouncer = bouncer::bouncer {};
        let result = mybouncer.sign_in().expect("");
        if result {
            interface.user_connected();
        }

    }
}


fn users_store(id: &String, main_pwd: &String) -> std::io::Result<()> {
    let mut extension: String = ".pwd".to_owned();
    let mut extension2: String = ".data".to_owned();
    let mut idToOwned: String = id.to_string();
    let mut idToOwned2: String = id.to_string();
    idToOwned.push_str(&extension);
    idToOwned2.push_str(&extension2);
    println!("Voici le pointeur id : {} ", &id);
    println!("Voici le pointeur ext : {}", &extension);
    let mut file = File::create(idToOwned)?;
    let mut file2 = File::create(idToOwned2)?;
    file.write_all(main_pwd.as_bytes()).expect("Echec d'écriture");
    Ok(())
}