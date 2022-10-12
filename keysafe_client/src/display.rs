use std::io;
use crate::login::Login;

use crate::user::{User, UserTrait};

pub trait Interface {
    fn display_menu(&self);
    fn display_create_account_success(&self);
    fn create_account(&self) -> Result<User, std::io::Error>;
    fn sign_in(&self) -> Result<User, std::io::Error>;
    fn user_connected(&self);
    fn main_menu(&self) -> Result<String, std::io::Error>;
    fn new_password(&self) -> Result<Login, std::io::Error>;
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

    fn sign_in(&self) -> Result<User, std::io::Error> {
        println!("display::TerminalInterface::sign_in");
        let mut pseudo = String::new();
        let mut mdp = String::new();
        println!("Pseudo :");
        io::stdin().read_line(&mut pseudo).expect(" ");
        println!("Mdp :");
        io::stdin().read_line(&mut mdp).expect(" ");
        Ok(User::new(pseudo.trim(), mdp.trim()))
    }

    fn user_connected(&self) {
        todo!()
    }

    fn main_menu(&self) -> Result<String, std::io::Error> {
        let mut choice = String::new();
        println!("Bravo! Vous êtes désormais connecté");
        println!("Que désirez-vous faire ?");
        println!("1. Accéder à mes mots de passe");
        println!("2. Ajouer un nouveau mot de passe");
        io::stdin().read_line( &mut choice).expect("");
        Ok(choice.trim().to_string())
    }

    fn new_password(&self) -> Result<Login, std::io::Error> {
        let mut login = Login {url: "".to_string(),mail: "".to_string(), pwd: "".to_string()};
        println!("Enregistrement d'un nouveau mot de passe...");
        println!("URL du site :");
        io::stdin().read_line( &mut login.url).expect("");
        println!("Adresse mail ou identifiant :");
        io::stdin().read_line(&mut login.mail).expect(" ");
        println!("Mot de passe :");
        io::stdin().read_line(&mut login.pwd).expect(" ");
        Ok(login)
    }
}
