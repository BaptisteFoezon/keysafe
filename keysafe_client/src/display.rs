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
    fn display_menu(&self) {
        println!("Bienvenue dans keysafe !");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
    }

    fn display_create_account_success() {
        println!("Compte créé avec succes");
    }

    fn  create_account(&self) ->  Result<user, std::io::Error>{

        let mut nom_in = String::new();
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte :");
        println!(". Nom ");
        io::stdin().read_line( &mut nom_in)?;
        println!(". Email ");
        io::stdin().read_line( &mut mail)?;
        println!(". mdp ");
        io::stdin().read_line( &mut mdp1)?;
        println!(". mdp ");
        io::stdin().read_line( &mut mdp2)?;
        Ok(user::new(nom_in.trim(), mail.trim(), mdp1.trim()))

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


