use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{fs, io};

use serde_derive::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::login::Login;
use crate::user::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginJSON {
    url: String,
    id: String,
    password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ListLogin {
    data: Vec<LoginJSON>,
}

pub trait JsonTrait {
    fn readListLogin(list_of_logins: ListLogin) -> String;
}

pub trait FileManagerTrait {
    fn create_user(user: User) -> io::Result<()>;
    fn users_store(id: String, main_pwd: String) -> io::Result<()>;
    fn data_store(user: User, login: Login) -> std::io::Result<()>;
    fn get_pwd_from_file(pseudo: &str) -> io::Result<String>;
    fn get_data_from_file(pseudo: &str) -> ListLogin;
}

pub struct FileManager {}

impl FileManagerTrait for FileManager {
    fn create_user(user: User) -> std::io::Result<()> {
        Self::users_store(user.pseudo, user.mdp)
    }

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
        let filename = id_to_owned;
        let tets = filename.clone();
        let file_content = fs::read_to_string(filename).expect("Échec d'ouverture du fichier");
        let mut file = OpenOptions::new()
            .write(true)
            .open(tets)
            .expect("Échec d'ouverture du fichier");

        if file_content == "" {
            let login_json = LoginJSON {
                url: login.url,
                id: login.mail,
                password: login.pwd,
            };

            let list_login = ListLogin {
                data: vec![login_json],
            };
            let j = serde_json::to_string(&list_login)?;
            file.write_all(j.as_bytes())?;
        } else {
            let mut list_logs: ListLogin =
                serde_json::from_str(&file_content).expect("Échec du parse JSON");

            let login_json = LoginJSON {
                url: login.url,
                id: login.mail,
                password: login.pwd,
            };

            list_logs.data.push(login_json);

            let j = serde_json::to_string(&list_logs)?;
            println!("{}", j);
            file.write_all(j.as_bytes())?;
        }

        Ok(())
    }

    fn get_pwd_from_file(pseudo: &str) -> io::Result<String> {
        let extension: String = ".pwd".to_owned();
        let mut id_to_owned: String = pseudo.to_string();
        id_to_owned.push_str(&extension);
        let result = fs::read_to_string(id_to_owned)?;
        Ok(result)
    }

    fn get_data_from_file(pseudo: &str) -> ListLogin {
        let extension: String = ".data".to_owned();
        let mut id_to_owned: String = pseudo.to_string();
        id_to_owned.push_str(&extension);
        let file_content = fs::read_to_string(id_to_owned).expect("Échec d'ouverture du fichier");
        let list_logs: ListLogin =
            serde_json::from_str(&file_content).expect("Échec du parse JSON");
        return list_logs;
    }
}

pub struct Json {}
impl JsonTrait for Json {
    fn readListLogin(list_of_logins: ListLogin) -> String {
        let mut all_logins: String = String::new();

        for (i, element) in list_of_logins.data.iter().enumerate() {
            let url: String = element.url.to_owned();
            let mail: String = element.id.to_owned();
            let pwd: String = element.password.to_owned();
            let logins_one_iteration: String = format!(
                "Site Web : {}\nIdentifiant :{}\nMot de passe : {}\n",
                url, mail, pwd
            );
            all_logins = format!("{} {}", all_logins, logins_one_iteration);
        }
        return all_logins;
    }
}
