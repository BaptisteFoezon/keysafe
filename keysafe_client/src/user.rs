use crate::file_manager::users_store;

pub trait UserTrait {
    fn new(pseudo: &str, mdp: &str) -> User;
    fn new_account(&self);
}

#[derive(Debug)]
pub struct User {
    pub pseudo: String,
    pub mdp: String,
}

impl UserTrait for User {
    fn new(name: &str, mdp: &str) -> User {
        User {
            pseudo: name.to_string(),
            mdp: mdp.to_string(),
        }
    }

    fn new_account(&self) {
        let pseudo = &self.pseudo;
        let  mdp = &self.mdp;
        users_store(pseudo.to_string(), mdp.to_string()).unwrap();
    }
}
