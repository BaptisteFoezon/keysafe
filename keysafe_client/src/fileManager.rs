use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::login::login;
use crate::user::{user, User};

pub fn createUser(user: user) -> std::io::Result<()> {
    users_store(user.pseudo, user.mdp)
}

pub fn getUserCredential() {}

pub fn users_store(id: String, main_pwd: String) -> std::io::Result<()> {
    let extension: String = ".pwd".to_owned();
    let extension2: String = ".data".to_owned();
    let mut id_to_owned: String = id.to_string();
    let mut id_to_owned2: String = id.to_string();
    id_to_owned.push_str(&extension);
    id_to_owned2.push_str(&extension2);
    println!("Voici le pointeur id : {} ", &id);
    println!("Voici le pointeur ext : {}", &extension);
    let mut file = File::create(id_to_owned)?;
    let mut file2 = File::create(id_to_owned2)?;
    file.write_all(main_pwd.as_bytes())
        .expect("Echec d'écriture");
    Ok(())
}

pub fn data_store(user: user, login: login) {
    let extension: String = ".data".to_owned();
    let mut id_to_owned: String = user.pseudo;
    id_to_owned.push_str(&extension);
    let mut file = OpenOptions::new().append(true).open(id_to_owned).unwrap();
    file.write_all(login.url.as_bytes()).expect("Echec d'écriture");
    file.write_all(login.mail.as_bytes()).expect("Echec d'écriture");
    file.write_all(login.pwd.as_bytes()).expect("Echec d'écriture");
}

pub fn get_pwd_from_file(pseudo: &str) -> String {
    let extension: String = ".pwd".to_owned();
    let mut id_to_owned: String = pseudo.to_string();
    id_to_owned.push_str(&extension);
    let contents = fs::read_to_string(id_to_owned).expect("  ");
    return contents;
}

