use crate::fileManager::users_store;

pub trait User {
    fn new(pseudo: &str, mdp: &str) -> user;
    fn new_account(&self);
}

#[derive(Debug)]
pub struct user {
    pub pseudo: String,
    pub mdp: String,
}

impl User for user {
    fn new(name: &str, mdp: &str) -> user {
        user {
            pseudo: name.to_string(),
            mdp: mdp.to_string(),
        }
    }

    fn new_account(&self) {
        let mut pseudo = &self.pseudo;
        let mut mdp = &self.mdp;
        users_store(pseudo.to_string(), mdp.to_string()).unwrap();
    }
}
