use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{fs, io};

use crate::login::Login;
use crate::user::User;

pub trait FileManagerTrait {
    fn create_user(user: User) -> io::Result<()>;
    fn get_user_credential();
    fn users_store(id: String, main_pwd: String) -> io::Result<()>;
    fn data_store(user: User, login: Login) -> std::io::Result<()>;
    fn get_pwd_from_file(pseudo: &str) -> io::Result<String>;
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
        file.write_all(login.url.as_bytes())?;
        file.write_all(login.mail.as_bytes())?;
        file.write_all(login.pwd.as_bytes())?;
        Ok(())
    }

    fn get_pwd_from_file(pseudo: &str) -> io::Result<String> {
        let extension: String = ".pwd".to_owned();
        let mut id_to_owned: String = pseudo.to_string();
        id_to_owned.push_str(&extension);
        let result = fs::read_to_string(id_to_owned)?;
        Ok(result)
    }
}
