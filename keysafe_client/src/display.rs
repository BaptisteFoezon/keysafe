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

        let mut nom = String::new();
        let mut prenom = String::new();
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte :");
        io::stdin().read_line(&mut prenom).expect("Veuillez rentrez une valeur correct");
        println!(". Email :");
        io::stdin().read_line(&mut mail).expect("Veuillez rentrez une valeur correct");
        println!(". Mot de passe :");
        io::stdin().read_line(&mut mdp1).expect("Veuillez rentrez une valeur correct");
        println!(". Confirmez votre mot de passe :");
        io::stdin().read_line(&mut mdp2).expect("Veuillez rentrez une valeur correct");
        return user::new(nom, mail, mdp1)

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
    fn new(name: String, email: String, mdp : String) -> user;

}

pub struct user{
    pub name : String,
    pub email : String,
    pub mdp : String,

}

impl User for user{
    fn new(name :String , email :String, mdp : String) -> user{
        user{
            name,
            email,
            mdp
        }
    }

}

