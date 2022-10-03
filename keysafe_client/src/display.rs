use std::io;
use crate::user::{User, user};

pub trait Interface {
    fn display_menu(&self);
    fn display_create_account_success();
    fn create_account(&self) -> Result<user, std::io::Error>;
}

pub struct Terminal_Interface{}

pub struct GUI_Interface{}


impl Interface for Terminal_Interface{
    fn display_menu(&self) -> Result<u8, std::io::Error>;{
        let mut display_menu_choice  u8;
        println!("Bienvenue dans keysafe !");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
        io::stdin().read_line(&mut display_menu_choice)?;
    }

    fn display_create_account_success() {
        println!("Compte créé avec succès !");
    }

    fn  create_account(&self) ->  Result<user, std::io::Error>{

        let mut nom_in = String::new();
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte...");
        println!("Pseudo :");
        io::stdin().read_line( &mut nom_in)?;
        println!("Adresse mail :");
        io::stdin().read_line( &mut mail)?;
        println!("Mot de passe :");
        io::stdin().read_line( &mut mdp1)?;
        println!("Mot de passe :");
        io::stdin().read_line( &mut mdp2)?;
        Ok(user::new(nom_in.trim(), mail.trim(), mdp1.trim()))

    }

    fn user_connected(&self) {
        println!("Vous êtes désormais connecté.");
    }

    fn main_menu(&self) ->  Result<user, std::io::Error>{
        let mut main_menu_choice u8;
        println!("Que voulez vous faire ?");
        println!("1. Consulter mes mots de passe enregistrés");
        println!("2. Enregistrer un nouveau mot de passe");
    }

    fn new_password(&self) -> Result<login, std::io::Error>{
        let mut url = String::new();
        let mut mail = String::new();
        let mut pwd = String::new();
        println!("Indiquez le nom du site :");
        io::stdin().read_line(&mut url)?;
        println!("Indiquez votre identifiant :");
        io::stdin().read_line(&mut mail)?;
        println!("Indiquez votre mot de passe :");
        io::stdin().read_line(&mut pwd)?;
    }
}




impl Interface for GUI_Interface{
    fn display_menu(&self) {
        println!("GUI")
    }

    fn display_create_account_success() {
        todo!()
    }

    fn create_account(&self )-> Result<user, std::io::Error> {
        todo!()
    }
}


