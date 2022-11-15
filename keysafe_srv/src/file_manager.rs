use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{fs, io};

use serde_derive::{Deserialize, Serialize};

use crate::login::Login;
use crate::user::User;

#[derive(Deserialize, Serialize, Debug)]
struct LoginJSON {
    url : String,
    id : String,
    password : String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ListLogin {
     list: Vec<LoginJSON>,
 }

pub trait FileManagerTrait {
    fn create_user(user: User) -> io::Result<()>;
    fn get_user_credential();
    fn users_store(id: String, main_pwd: String) -> io::Result<()>;
    fn data_store(user: User, login: Login) -> std::io::Result<()>;
    fn get_pwd_from_file(pseudo: &str) -> io::Result<String>;
    fn get_data_from_file() -> ListLogin;
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
        dbg!("Voici le pointeur id : {} ", &id);
        dbg!("Voici le pointeur ext : {}", &extension);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(id_to_owned)?;
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(id_to_owned2)?;
        file.write_all(main_pwd.as_bytes())?;
        Ok(())
    }

    fn data_store(user: User, login: Login) -> std::io::Result<()> {
        let extension: String = ".data".to_owned();
        let mut id_to_owned: String = user.pseudo;
        id_to_owned.push_str(&extension);
        let mut file = OpenOptions::new().append(true).open(id_to_owned)?;
        // file.write_all(login.url.as_bytes())?;
        // file.write_all(login.mail.as_bytes())?;
        // file.write_all(login.pwd.as_bytes())?;
        let login_json = LoginJSON{url: login.url, id: login.mail, password: login.pwd};
        let j = serde_json::to_string(&login_json)?;
        println!("On est rentré dans datastore");
        println!("j= {}", j);
        file.write_all(j.as_bytes())?; 

        Ok(())
    }

    fn get_pwd_from_file(pseudo: &str) -> io::Result<String> {
        let extension: String = ".pwd".to_owned();
        let mut id_to_owned: String = pseudo.to_string();
        id_to_owned.push_str(&extension);
        let result = fs::read_to_string(id_to_owned)?;
        Ok(result)
    }

    fn get_data_from_file() -> ListLogin 
    {
            let file_content = fs::read_to_string("test.json").expect("Echec ouverture fichier");
            let list_logs: ListLogin = serde_json::from_str(&file_content).unwrap();
            return list_logs;
    
        }
}
