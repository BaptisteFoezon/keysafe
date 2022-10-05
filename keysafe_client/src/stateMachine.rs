use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str;

use pwhash::bcrypt;

use crate::display::Terminal_Interface;
use crate::Interface;
use crate::stateMachine::State::{Logged, LogOut};

pub(crate) enum State {
    LogOut,
    SignUp,
    Logged,
}


pub struct SM {
    pub(crate) state: State,
    interface: Terminal_Interface,
}

impl SM {
    // when StateMachine is created state -> LogOut
    pub fn new(interface: Terminal_Interface) -> SM {
        SM {
            state: LogOut,
            interface: interface,
        }
    }
    pub fn ask_sign_in(&mut self) -> () {
        match self.state {
            LogOut => {
                self.interface.sign_in();
                self.state = Logged;
            }
            _ => println!("pas le droit")
        }
    }

    pub fn ask_sign_up(&mut self) -> () {
        match self.state {
            LogOut => {
                let user = self.interface.create_account().expect("TODO: panic message");
                self.state = Logged;
                println!("{:?}", user);
                let mdp_hash = bcrypt::hash(user.mdp).unwrap();
                users_store(&user.pseudo, &mdp_hash).expect("Une erreur est survenue");
                println!("mot de pass hasher : {}", mdp_hash);
            }
            _ => println!("pas le droit")
        }
    }

    pub fn logout(&mut self) -> () {
        match self.state {
            Logged => {
                println!("logOut");
                self.state = LogOut
            }
            _ => {
                println!("jdsqdqs");
            }
        }
    }

    pub fn printMenu(&mut self) -> () {
        match self.state {
            LogOut => {
                let mut choice = String::new();
                self.interface.display_menu();
                io::stdin().read_line(&mut choice).expect("mauvaise saisie");
                if choice.trim().eq("1") {
                    self.ask_sign_up();
                }
            }
            _ => println!("opération impossible")
        }
    }
}


fn users_store(id: &String, mdp: &String) -> std::io::Result<()> {
    //let mut file = OpenOptions::new().write(true).append(true).open("users.txt").expect("Unable to open file");
    let extension: String = ".txt".to_owned();
    let mut id_owned: String = id.to_owned();
    id_owned.push_str(&extension);
    println!("Voici le pointeur id : {} ", id_owned);
    println!("Voici le pointeur ext : {}", &extension);
    let mut file = File::create(id_owned)?;
    file.write_all(mdp.as_bytes()).expect("Echec d'écriture");
    Ok(())
}