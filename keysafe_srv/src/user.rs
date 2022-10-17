use crate::file_manager::{FileManager, FileManagerTrait};

pub trait UserTrait {
    fn new(pseudo: &str, mdp: &str) -> User;
    fn new_account(&self) -> std::io::Result<()>;
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

    fn new_account(&self) -> std::io::Result<()> {
        let pseudo = &self.pseudo;
        let  mdp = &self.mdp;
        let result = FileManager::users_store(pseudo.to_string(), mdp.to_string())?;
        Ok(result)
    }
}
