extern crate core;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str;

use pwhash::bcrypt;

use display::Interface;

mod display;
mod user;


enum State {
    Init,
    Login,
    LogOut,
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Init
        }
    }
    fn init(&mut self) {
        self.state = match self.state {
            State::Init => State::LogOut,
            _ => panic!("dsqdq")
        }
    }
    fn log_in(&mut self) {
        self.state = match self.state {
            State::LogOut => State::Login,
            _ => panic!("Transition impossible")
        }
    }
    fn log_out(&mut self) {
        self.state = match self.state {
            State::Login => State::LogOut,
            _ => panic!("Transition impossible")
        }
    }
}

fn main() {
    let mut stateMachine = StateMachine::new(); //create
    // 'my_machine' is destroyed when it falls of the scope
    stateMachine.init();
    stateMachine.log_in();

    let interface = display::Terminal_Interface {};
    interface.display_menu();
    let user = interface.create_account().expect("dsqdsq");
    println!("{:?}", user);
    let mdp_hash = bcrypt::hash(user.mdp).unwrap();
    users_store(&user.pseudo, &mdp_hash).expect("Une erreur est survenue");
    println!("mot de pass hasher : {}", mdp_hash);
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
