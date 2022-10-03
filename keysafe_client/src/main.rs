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

struct User<'a>{
    nom : &'a str,
    prenom : &'a str,
    email : &'a str,
    mdp : &'a str,
}

impl User<'a>{
    fn new(nom : &str , prenom : &str, email : &str, mdp : &str)-> User{
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
    let mut extension : String = ".txt".to_owned(); 
    let mut copy = id.to_owned();
    copy.push_str(&extension);
    println!("Voici le nom du fichier : {}", copy);
    let mut file = File::create(copy)?;
    file.write_all(mdp.as_bytes()).expect("Échec d'écriture");
    //file.write_all(nom<).expect("Echec d'écriture");
    Ok(())
}

}