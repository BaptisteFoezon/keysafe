mod display;
use std::io;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::str;

use pwhash::bcrypt;
use display::Interface;
use crate::display::{user, User};

fn main() {
    let interface = display::Terminal_Interface{};
   interface.display_menu();
    let user = interface.create_acount();
    let mdp_hash = bcrypt::hash(user.mdp).unwrap();
    users_store(&user.email, &mdp_hash);
    println!("mot de pass hasher : {}", mdp_hash );

struct User{
    nom : String,
    prenom : String,
    email : String,
    mdp : String,
}

impl User{
    fn new(nom : String, prenom : String, email : String, mdp : String)-> User{
        User{
            nom,
            prenom,
            email,
            mdp
        }
    }
    fn get_name(&self) -> &str { &self.nom }
    fn get_mail(&self) -> &str{ &self.email }
}

fn users_store(id: &String, mdp: &String) -> std::io::Result<()> {
    //let mut file = OpenOptions::new().write(true).append(true).open("users.txt").expect("Unable to open file");
    let extension : String = ".txt".to_owned();
    id.to_owned().push_str(&extension);
    println!("Voici le pointeur id : {} ", &id);
    println!("Voici le pointeur ext : {}", &extension);
    let mut file = File::create(id)?;
    file.write_all(mdp.as_bytes()).expect("Echec d'écriture");
    Ok(())
}

}