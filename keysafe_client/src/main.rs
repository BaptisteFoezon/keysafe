use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str;

use pwhash::bcrypt;

use display::Interface;

//use login::login;

mod display;
mod user;
mod login;

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
        signing_in();
    } else {
        interface.user_connected();
        let mut main_choice = interface.main_menu().expect("  ");
        if main_choice == "2" {
            let login1 = interface.new_password().expect("  ");
            data_store(user, login1);
            
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

fn data_store(user: user, data_to_store: login) -> std::io::Result<()> {
    let mut extension: String = ".data".to_owned();
    let mut idToOwned: String = user::name.to_string();
    let mut file = OpenOptions::new().append(true).open(idToOwned).unwrap();
    file.write_all(login::url.as_bytes()).expect("Echec d'écriture");
    file.write_all(login::mail.as_bytes()).expect("Echec d'écriture");
    file.write_all(login::pwd.as_bytes()).expect("Echec d'écriture");
    OK(())
}

fn signing_in() -> std::io::Result<()> {
let mut user = interface.sign_in().expect("  ");
let mut extension: String = ".pwd".to_owned();
let mut idToOwned: String = user::name.to_string();
let mut file = OpenOptions::new().append(true).open(idToOwned).unwrap();
let mdp_hash = bcrypt::hash(user.mdp).unwrap();
let contents = fs::read_to_string(idToOwned).expect("  ");
if (contents.eq(mdp_hash)){
    interface.user_connected;
}
else {println!("Identifiant ou mot de passe incorrect");}
OK(())

}