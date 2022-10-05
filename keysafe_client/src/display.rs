use std::io;
use std::io::Error;

use crate::login::{Login, login};
use crate::user::{User, user};

pub trait Interface {
    fn display_menu(&self) -> Result<String, std::io::Error>;
    fn display_create_account_success(&self);
    fn create_account(&self) -> Result<user, std::io::Error>;
    fn sign_in(&self);
    fn user_connected(&self);
    fn main_menu(&self) -> Result<String, std::io::Error>;
    fn new_password(&self) -> Result<login, std::io::Error>;
}

pub struct Terminal_Interface {}

pub struct GUI_Interface {}


impl Interface for Terminal_Interface {
    fn display_menu(&self) -> Result<String, std::io::Error> {
        let mut display_menu_choice = String::new();
        println!("Bienvenue dans keysacafe !");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
        io::stdin().read_line(&mut display_menu_choice)?;
        Ok(display_menu_choice.trim().to_string())
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
        io::stdin().read_line(&mut pseudo);
        println!("Mot de passe :");
        io::stdin.read_line(&mut mdp);
        Ok(user::new(pseudo.trim(), mail.trim(), mdp.trim()))
    }

    fn user_connected(&self) {
        println!("Vous êtes désormais connecté.");
    }

    fn main_menu(&self) -> Result<String, std::io::Error> {
        let mut var = String::new();
        println!("Que voulez vous faire ?");
        println!("1. Consulter mes mots de passe enregistrés");
        println!("2. Enregistrer un nouveau mot de passe");
        io::stdin().read_line(&mut var)?;
        Ok(var.trim().to_string())
    }

    fn new_password(&self) -> Result<login, std::io::Error> {
        let mut url = String::new();
        let mut mail = String::new();
        let mut pwd = String::new();
        println!("Indiquez le nom du site :");
        io::stdin().read_line(&mut url)?;
        println!("Indiquez votre identifiant :");
        io::stdin().read_line(&mut mail)?;
        println!("Indiquez votre mot de passe :");
        io::stdin().read_line(&mut pwd)?;
        Ok(login::new(url.trim(), mail.trim(), pwd.trim()))
    }
}



