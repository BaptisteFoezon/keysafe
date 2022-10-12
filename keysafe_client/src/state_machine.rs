use std::io;
use std::io::{Error, ErrorKind};

use pwhash::bcrypt;

use crate::{BouncerTrait, Interface};
use crate::bouncer::Bouncer;
use crate::display::TerminalInterface;
use crate::file_manager::{FileManager, FileManagerTrait};
use crate::state_machine::State::{IDLE, Logged, LogOut};
use crate::user::{User, UserTrait};

pub(crate) enum State {
    IDLE,
    LogOut,
    Logged,
}

pub struct SM {
    pub(crate) state: State,
    interface: TerminalInterface,
}

impl SM {
    // when StateMachine is created state -> LogOut
    pub fn new(interface: TerminalInterface) -> SM {
        SM {
            state: IDLE,
            interface,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            IDLE => {
                self.state = LogOut;
                self.print_menu();
            }
            _ => println!("pas le droit"),
        }
    }

    fn logged_menu(&mut self, user: User) -> () {
        match self.state {
            Logged => {
                self.interface.print_main_menu();
                let choice = self.interface.ask_choice();
                match choice {
                    Ok(value) => {
                        if value.eq("1"){
                            self.see_password(user);
                        }
                        else if value.eq("2") {
                            self.add_password(user);
                        }
                        else {
                            self.logged_menu(user);
                        }
                    }
                    Err(error) => {
                        dbg!("{}", error.to_string());
                        self.logged_menu(user);
                    }
                }
            }
            _ => println!("logged_menu :: transition depuis"),
        }
    }

    fn ask_sign_in(&mut self) -> () {
        match self.state {
            LogOut => {
                let user = self.interface.sign_in().expect("TODO: panic message");
                let bouncer = Bouncer::new();
                let sign_result = bouncer.sign_in(&*user.pseudo, &*user.mdp);
                match sign_result {
                    Ok(val) => {
                        if val {
                            self.state = Logged;
                            self.logged_menu(user);
                        } else {
                            println!("mot de passe incorect ");
                            self.print_menu();
                        }
                    }
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => {
                            println!("Aucun utilisateur de ce nom existe, veuillez créer un \
                            compte ");
                            self.print_menu();
                        }
                        _ => {
                            panic!("une erreur est surnenu")
                        }
                    },
                }
            }
            _ => println!("pas le droit"),
        }
    }

    fn see_password(&mut self, user: User) -> () {
        match self.state {
            Logged => {
                println!("vous avez demander à voir vos mots de passe");
                println!("{}", user.pseudo)
            }
            _ => println!("vous netes pas connecté")
        }
    }


    fn add_password(&mut self, user: User) -> () {
        match self.state {
            Logged => {
                println!("vous avez demander à ajouter un mot de passe");
                let login = self.interface.new_password().unwrap();
                FileManager::data_store(user, login);
            }
            _ => println!("vous netes pas connecté")
        }
    }
    fn ask_sign_up(&mut self) -> () {
        match self.state {
            LogOut => {
                dbg!("ask_sign_up from LogOut state");
                let mut user = self
                    .interface
                    .create_account()
                    .expect("TODO: panic message");
                dbg!("{}", &user);
                user.mdp = bcrypt::hash(user.mdp).unwrap();
                let result = user.new_account();
                match result {
                    Ok(_) => {
                        dbg!("{}", &user);
                        self.state = Logged;
                        self.logged_menu(user);
                    }
                    Err(error) => match error.kind() {
                        ErrorKind::AlreadyExists => {
                            println!("Un utilisateur du meme nom existe veuillez changer de \
                            pseudo");
                            self.ask_sign_up();
                        }
                        _ => {
                            dbg!("{}", error.to_string());
                            panic!("Une erreur est survenue")
                        }
                    }
                }
            }
            _ => println!("pas le droit"),
        }
    }

    fn logout(&mut self) -> () {
        match self.state {
            Logged => {
                println!("logOut");
                self.state = LogOut
            }
            _ => {
                println!("logOut");
                self.logout();
            }
        }
    }

    fn print_menu(&mut self) -> () {
        match self.state {
            LogOut => {
                let mut choice = String::new();
                self.interface.display_menu();
                io::stdin().read_line(&mut choice).expect("mauvaise saisie");
                println!("dqdqsd");
                if choice.trim().eq("1") {
                    println!("print_menu :: vous avez demander à sign up ");
                    self.ask_sign_up();
                }
                if choice.trim().eq("2") {
                    println!("print_menu :: vous avez demander à sign in ");
                    self.ask_sign_in();
                }
            }
            _ => println!("opération impossible"),
        }
    }

    fn add_new_log(&mut self) -> () {
        match self.state {
            Logged => {
                self.interface.new_password().expect("TODO: panic message");
            }
            _ => println!("opération impossible"),
        }
    }
}
