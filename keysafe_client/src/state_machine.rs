use std::io;
use pwhash::bcrypt;

use crate::bouncer::bouncer;
use crate::display::TerminalInterface;
use crate::display::Terminal_Interface;
use crate::fileManager::data_store;
use crate::state_machine::State::{LogOut, Logged, IDLE};
use crate::user::{User};
use crate::{Bouncer, Interface};

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

    fn logged_menu(&mut self, user: user) -> () {
        match self.state {
            Logged => {
                let choice = self.interface.main_menu().expect("");
                if choice.eq("2") {
                    let login = self.interface.new_password().unwrap();
                    data_store(user, login);
                }
            }
            _ => println!("logged_menu :: transition depuis"),
        }
    }

    fn ask_sign_in(&mut self) -> () {
        match self.state {
            LogOut => {
                let user = self.interface.sign_in().expect("TODO: panic message");
                let bouncer = bouncer::new();
                let sign_result = bouncer.sign_in(&*user.pseudo, &*user.mdp).expect(
                    "TODO: panic \
                message",
                );
                if sign_result {
                    self.state = Logged;
                    self.logged_menu(user);
                } else {
                    println!("mot de passe incorect ")
                }
            }
            _ => println!("pas le droit"),
        }
    }

    fn ask_sign_up(&mut self) -> () {
        match self.state {
            LogOut => {
                println!("ask_sign_up from LogOut state");
                let mut user = self
                    .interface
                    .create_account()
                    .expect("TODO: panic message");
                self.state = Logged;
                println!("{:?}", user);
                user.mdp = bcrypt::hash(user.mdp).unwrap();
                user.new_account();
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
                self.interface.new_password();
            }
            _ => println!("opération impossible"),
        }
    }
}
