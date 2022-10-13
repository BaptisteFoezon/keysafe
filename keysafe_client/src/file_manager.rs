use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::login::{Login};
use crate::user::User;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]


struct Data {
    url : String,
    id : String,
    password : String
}

pub trait FileManagerTrait {
    fn create_user(user: User) -> std::io::Result<()>;
    fn get_user_credential();
    fn users_store(id: String, main_pwd: String) -> std::io::Result<()>;
    fn data_store(user: User, login: Login);
    fn get_pwd_from_file(pseudo: &str) -> String;
    fn get_data_from_file() -> Login;
}

pub struct FileManager {}


impl FileManagerTrait for FileManager {
    fn create_user(user: User) -> std::io::Result<()> {
        Self::users_store(user.pseudo, user.mdp)
    }

    fn get_user_credential() {}

    fn users_store(id: String, main_pwd: String) -> std::io::Result<()> {
        let extension: String = ".pwd".to_owned();
        let extension2: String = ".data".to_owned();
        let mut id_to_owned: String = id.to_string();
        let mut id_to_owned2: String = id.to_string();
        id_to_owned.push_str(&extension);
        id_to_owned2.push_str(&extension2);
        println!("Voici le pointeur id : {} ", &id);
        println!("Voici le pointeur ext : {}", &extension);
        let mut file = File::create(id_to_owned)?;
        File::create(id_to_owned2)?;
        file.write_all(main_pwd.as_bytes())
            .expect("Echec d'écriture");
        Ok(())
    }

    fn data_store(user: User, login: Login) {
        let extension: String = ".data".to_owned();
        let mut id_to_owned: String = user.pseudo;
        id_to_owned.push_str(&extension);
        let mut file = OpenOptions::new().append(true).open(id_to_owned).unwrap();
        file.write_all(login.url.as_bytes()).expect("Echec d'écriture");
        file.write_all(login.mail.as_bytes()).expect("Echec d'écriture");
        file.write_all(login.pwd.as_bytes()).expect("Echec d'écriture");
    }

    fn get_pwd_from_file(pseudo: &str) -> String {
        let extension: String = ".pwd".to_owned();
        let mut id_to_owned: String = pseudo.to_string();
        id_to_owned.push_str(&extension);
        let contents = fs::read_to_string(id_to_owned).expect("  ");
        return contents;
    }



    fn get_data_from_file() -> Login 
{
        let file_content = fs::read_to_string("test.json").expect("Echec ouverture fichier");
        let data: Data = serde_json::from_str(&file_content).expect("JSON was not well-formatted");
        println!("{:?}", data);
        return Login { url: "test".to_string(), mail: "test".to_string(), pwd: "test".to_string() }

    }
}


