use pwhash::bcrypt;
use std::io;
use std::io::ErrorKind;

use crate::bouncer::{Bouncer, BouncerTrait};
use crate::display::{TcpInterface, TcpInterfaceTrait};
use crate::file_manager::{FileManager, FileManagerTrait};
use crate::state_machine::State::{IDLE, Logged, LogOut};
use crate::user::{User, UserTrait};

pub(crate) enum State {
    IDLE,
    LogOut,
    Logged,
}

pub trait StateMachineTrait {
    fn new(tcp: TcpInterface) -> SM;
    fn start(&mut self);
    fn logout(&mut self);
    fn logged_menu(&mut self, user: User);
    fn print_menu(&mut self);
    fn ask_sign_in(&mut self);
    fn ask_sign_up(&mut self);
    fn see_password(&mut self, user: User);
    fn add_new_log(&mut self, user: User);
}

pub struct SM {
    pub(crate) state: State,
    pub interface: TcpInterface,
}

impl StateMachineTrait for SM {
    // when StateMachine is created state -> LogOut
    fn new(interface: TcpInterface) -> SM {
        SM {
            state: IDLE,
            interface,
        }
    }

    fn start(&mut self) {
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
                        if value.eq("1") {
                            self.see_password(user);
                        } else if value.eq("2") {
                            self.add_new_log(user);
                        } else {
                            self.logged_menu(user);
                        }
                    }
                    Err(error) => {
                        dbg!("{}", error.to_string());
                        self.logged_menu(user);
                    }
                }
            }
            _ => println!("logged_menu :: transition impossible"),
        }
    }

    fn ask_sign_in(&mut self) -> () {
        match self.state {
            LogOut => {
                let user = self.interface.sign_in();
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
                            println!(
                                "Aucun utilisateur de ce nom existe, veuillez créer un \
                            compte "
                            );
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
            _ => println!("vous netes pas connecté"),
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
                            println!(
                                "Un utilisateur du meme nom existe veuillez changer de \
                            pseudo"
                            );
                            self.ask_sign_up();
                        }
                        _ => {
                            dbg!("{}", error.to_string());
                            panic!("Une erreur est survenue")
                        }
                    },
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
                println!("print_menu from LogOut state");
                let mut choice = String::new();
                self.interface.display_menu();
                let result = io::stdin().read_line(&mut choice);
                match result {
                    Ok(_) => {}
                    Err(_) => {
                        println!("une erreur est survenue");
                        self.print_menu();
                    }
                }
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

    fn add_new_log(&mut self, user: User) -> () {
        match self.state {
            Logged => {
                let result = self.interface.new_password();
                match result {
                    Ok(login) => {
                        let result = FileManager::data_store(user, login);
                        match result {
                            Ok(_) => {
                                println!("mdp enregistrer avec succes")
                            }
                            Err(_) => {
                                println!("Une erreur est survenue veuillez réesayer");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Une erreur est survenue merci de réesayer");
                        self.add_new_log(user)
                    }
                }
            }
            _ => println!("opération impossible"),
        }
    }
}
