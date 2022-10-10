use std::io;
use std::io::Error;

use crate::login::{Login, login};
use crate::user::{User, user};

pub trait Interface {
    fn display_menu(&self);
    fn display_create_account_success(&self);
    fn create_account(&self) -> Result<user, std::io::Error>;
    fn sign_in(&self)-> Result<user, std::io::Error>;
    fn user_connected(&self);
    fn main_menu(&self) -> Result<String, std::io::Error>;
    fn new_password(&self) -> Result<login, std::io::Error>;
}

pub struct Terminal_Interface {}

pub struct GUI_Interface {}


impl Interface for Terminal_Interface {
    fn display_menu(&self)  {
        let mut display_menu_choice = String::new();
        println!("Bienvenue dans keysacafe !");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
    }

    fn display_create_account_success(&self) {
        println!("Compte créé avec succès !");
    }

    fn create_account(&self) -> Result<user, std::io::Error> {

        let mut nom_in = String::new();
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte...");
        println!("Pseudo :");
        io::stdin().read_line(&mut nom_in)?;
        println!("Adresse mail :");
        io::stdin().read_line(&mut mail)?;
        println!("Mot de passe :");
        io::stdin().read_line(&mut mdp1)?;
        println!("Mot de passe :");
        io::stdin().read_line(&mut mdp2)?;
        Ok(user::new(nom_in.trim(), mail.trim(), mdp1.trim()))
    }

    fn sign_in(&self) -> Result<user, std::io::Error> {
        let mut pseudo = String::new();
        let mut mail = String::new();
        let mut mdp = String::new();
        println!("Veuillez vous identifier...");
        println!("Pseudo :");
        io::stdin().read_line(&mut pseudo).expect(" ");
        println!("Mdp :");
        io::stdin().read_line(&mut mdp).expect(" ");
        Ok(user::new(&pseudo, "* &str */", &mdp))

    }

    fn user_connected(&self) {
        todo!()
    }

    fn main_menu(&self) -> Result<String, Error> {
        todo!()
    }

    fn new_password(&self) -> Result<login, Error> {
        todo!()
    }
}




