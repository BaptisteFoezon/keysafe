use crate::Interface;
use crate::stateMachine::State::{Logged, LogOut};

pub(crate) enum State {
    LogOut,
    SignUp,
    Logged,
}


pub struct SM {
    pub(crate) state: State,
    interface : Interface::,
}

impl SM {
    // when StateMachine is created state -> LogOut
    pub fn new() -> SM{
        SM{
            state : LogOut,
        }
    }
    pub fn ask_sign_up(&mut self) -> () {
        match self.state {
            LogOut => {
                println!("sign up");
                println!("je demande les infos de sign up");
                self.state = Logged;
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

    pub fn printMenu(&mut self) -> (){
        match self.state {
            _ => {
                println!("demande du choix ");
                println!("L'utilisateur choisit 1 (demande de sign up)");
                self.ask_sign_up();
            }
        }
    }
}