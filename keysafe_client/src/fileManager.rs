use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::login::login;
use crate::user::{user, User};

pub fn createUser(user : user) -> std::io::Result<()> {
    users_store( user.pseudo,  user.mdp)
}

pub fn getUserCredential(){

}



pub fn users_store(id: String, main_pwd: String) -> std::io::Result<()> {
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

pub fn data_store(user: user, url : &str , pwd: &str) {
    let mut extension: String = ".data".to_owned();
    let mut idToOwned: String = user.pseudo;
    let mut file = OpenOptions::new().append(true).open(idToOwned).unwrap();
    file.write_all(url.as_bytes()).expect("Echec d'écriture");
    file.write_all(pwd.as_bytes()).expect("Echec d'écriture");
}

pub fn getPwdFromFile(pseudo : &str) -> String{
    let mut extension: String = ".pwd".to_owned();
    let mut idToOwned: String = pseudo.to_string();
    let contents = fs::read_to_string(idToOwned).expect("  ");
    return contents;
}