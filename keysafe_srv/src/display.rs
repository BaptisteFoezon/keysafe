use std::io;

use crate::login::Login;
use crate::user::{User, UserTrait};
use crate::file_manager::{ListLogin, JsonTrait, Json};

pub trait Interface {
    fn display_menu(&self);
    fn display_create_account_success(&self);
    fn create_account(&self) -> Result<User, std::io::Error>;
    fn sign_in(&self) -> User;
    fn user_connected(&self);
    fn print_main_menu(&self);
    fn ask_choice(&self) -> Result<String, std::io::Error>;
    fn new_password(&self) -> Result<Login, std::io::Error>;
    fn display_all_pwd(&self, list_mdp: ListLogin);
}

pub struct TerminalInterface {}

impl Interface for TerminalInterface {
    fn display_menu(&self) {
        println!("Bienvenue dans keysacafe !");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
    }

    fn display_create_account_success(&self) {
        println!("Compte créé avec succès !");
    }

    fn create_account(&self) -> Result<User, std::io::Error> {
        let mut nom_in = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte...");
        println!("Pseudo :");
        io::stdin().read_line(&mut nom_in)?;
        println!("Mot de passe :");
        io::stdin().read_line(&mut mdp1)?;
        println!("Mot de passe :");
        io::stdin().read_line(&mut mdp2)?;
        Ok(User::new(nom_in.trim(), mdp1.trim()))
    }

    fn sign_in(&self) -> User {
        println!("display::TerminalInterface::sign_in");
        let mut pseudo = String::new();
        let mut mdp = String::new();
        println!("Pseudo :");
        let result = io::stdin().read_line(&mut pseudo);
        match result {
            Ok(..) => {
                if pseudo.trim() == "" {
                    println!("le pseudo ne peut pas etre vide");
                    self.sign_in()
                } else {
                    println!("Mdp :");
                    let mdp_result = io::stdin().read_line(&mut mdp);
                    match mdp_result {
                        Ok(_) => User::new(pseudo.trim(), mdp.trim()),
                        Err(_) => self.sign_in(),
                    }
                }
            }
            Err(_) => self.sign_in(),
        }
    }

    fn user_connected(&self) {
        todo!()
    }

    fn print_main_menu(&self) {
        println!("Que désirez-vous faire ?");
        println!("1. Accéder à mes mots de passe");
        println!("2. Ajouer un nouveau mot de passe");
    }

    fn ask_choice(&self) -> Result<String, std::io::Error> {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("");
        Ok(choice.trim().to_string())
    }

    fn new_password(&self) -> Result<Login, std::io::Error> {
        let mut login = Login {
            url: "".to_string(),
            mail: "".to_string(),
            pwd: "".to_string(),
        };
        println!("Enregistrement d'un nouveau mot de passe...");
        println!("URL du site :");
        let mut url = String::new();
        let mut mail = String::new();
        let mut pwd = String::new();
        io::stdin().read_line(&mut url)?;
        login.url = url.trim().to_string();
        println!("Adresse mail ou identifiant :");
        io::stdin().read_line(&mut mail)?;
        login.mail = mail.trim().to_string();
        println!("Mot de passe :");
        io::stdin().read_line(&mut pwd)?;
        login.pwd = pwd.trim().to_string();
        Ok(login)
    }

    fn display_all_pwd(&self, list_mdp: ListLogin){
        println!("Voici vos mots de passe enregistrés :");
        let mut list_all_logins: String = Json::readListLogin(list_mdp);
        println!("{}", list_all_logins);
    }
}
