use std::io;

use crate::login::Login;
use crate::tcp::{MyTcp, MyTcpTrait};
use crate::user::{User, UserTrait};

pub trait TcpInterfaceTrait {
    fn new(tcp: MyTcp) -> TcpInterface;
    fn display_menu(&mut self);
    fn display_create_account_success(&mut self);
    fn create_account(&mut self) -> Result<User, std::io::Error>;
    fn sign_in(&mut self) -> User;
    fn user_connected(&mut self);
    fn print_main_menu(&mut self);
    fn ask_choice(&mut self) -> Result<String, std::io::Error>;
    fn new_password(&mut self) -> Result<Login, std::io::Error>;
}


pub struct TcpInterface {
    pub tcp: MyTcp,
}

impl TcpInterfaceTrait for TcpInterface {
    fn new(tcp: MyTcp) -> TcpInterface {
        TcpInterface { tcp }
    }

    fn display_menu(&mut self) {
        self.tcp.send("menuxwcwxcwxcwxcwxcwxcwxcwxcwxcw");
    }

    fn display_create_account_success(&mut self) {
        self.tcp.send("compte créé avec succes");
    }

    fn create_account(&mut self) -> Result<User, std::io::Error> {
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

    fn sign_in(&mut self) -> User {
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

    fn user_connected(&mut self) {
        todo!()
    }

    fn print_main_menu(&mut self) {
        println!("Que désirez-vous faire ?");
        println!("1. Accéder à mes mots de passe");
        println!("2. Ajouter un nouveau mot de passe");
    }

    fn ask_choice(&mut self) -> Result<String, std::io::Error> {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("");
        Ok(choice.trim().to_string())
    }

    fn new_password(&mut self) -> Result<Login, std::io::Error> {
        let mut login = Login {
            url: "".to_string(),
            mail: "".to_string(),
            pwd: "".to_string(),
        };
        println!("Enregistrement d'un nouveau mot de passe...");
        println!("URL du site :");
        io::stdin().read_line(&mut login.url)?;
        println!("Adresse mail ou identifiant :");
        io::stdin().read_line(&mut login.mail)?;
        println!("Mot de passe :");
        io::stdin().read_line(&mut login.pwd)?;
        Ok(login)
    }
}

