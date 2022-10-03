use std::io;
pub trait Interface {
    fn display_menu(&self);
    fn create_acount(&self) -> user;
}

pub struct Terminal_Interface{

}

pub struct GUI_Interface{

}


impl Interface for Terminal_Interface{
    fn display_menu(&self) {
        println!("Bienvenue dans keysafe !!");
        println!("1. Créer un compte");
        println!("2. Se connecter ");
    }

    fn  create_acount(&self) ->  user{

        let  nom : &str;
        let mut prenom : &str;
        let mut mail : &str;
        let mut mdp1 : &str;
        let mut mdp2 : &str;
        println!("Création de votre compte :");
        io::stdin().read_line(&mut prenom).expect("Veuillez rentrez une valeur correct");
        println!(". Email :");
        io::stdin().read_line(&mut mail).expect("Veuillez rentrez une valeur correct");
        println!(". Mot de passe :");
        io::stdin().read_line(&mut mdp1).expect("Veuillez rentrez une valeur correct");
        println!(". Confirmez votre mot de passe :");
        io::stdin().read_line(&mut mdp2).expect("Veuillez rentrez une valeur correct");
        return user::new(nom.trim() , mail.trim(), mdp1.trim())

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

pub trait User{
    fn new(name: &str, email: &str, mdp : &str) -> user;

}

pub struct user{
    pub name : &str;
    pub email : &str;
    pub mdp : &str,

}

impl User for user{
    fn new(name :&str , email :&str, mdp : &str) -> user{
        user{
            name,
            email,
            mdp
        }
    }

}

