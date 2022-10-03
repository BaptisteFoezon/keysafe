use std::io;
use crate::user::{User, user};

pub trait Interface {
    fn display_menu(&self);
    fn create_acount(&self) -> user;
}

pub struct Terminal_Interface{}

pub struct GUI_Interface{}


impl Interface for Terminal_Interface{
    fn display_menu(&self) {
        println!("Bienvenue dans keysafe !!");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
    }

    fn  create_acount(&self) ->  user{

        let  nom_in = &mut "".to_string();
        let  prenom = &mut "".to_string();
        let  mail = &mut "".to_string();
        let  mdp1 = &mut "".to_string();
        let  mdp2 = &mut "".to_string();
        println!("Création de votre compte :");
        println!(". Nom ");
        io::stdin().read_line( nom_in).expect("Veuillez rentrez une valeur correct");
        //f.write_all(b"Hello, world! ")?;
        println!(". Prénom ");
        io::stdin().read_line( prenom).expect("Veuillez rentrez une valeur correct");
        println!(". Email ");
        io::stdin().read_line( mail).expect("Veuillez rentrez une valeur correct");
        println!(". mdp ");
        io::stdin().read_line( mdp1).expect("Veuillez rentrez une valeur correct");
        println!(". mdp ");
        io::stdin().read_line( mdp2).expect("Veuillez rentrez une valeur correct");
        return user::new(nom_in.trim().to_string(), mail.trim().to_string(), mdp1.trim().to_string())

    }
}


impl Interface for GUI_Interface{
    fn display_menu(&self) {
        println!("GUI")
    }

    fn create_acount(&self )-> user {
        todo!()
    }
}


