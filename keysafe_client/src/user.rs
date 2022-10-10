use pwhash::bcrypt;
use crate::fileManager::getPwdFromFile;

pub trait User{
    fn new(pseudo: &str, mail: &str, mdp : &str) -> user;
    fn sign_in(pseudo: &str, mdp: &str) -> Result<bool, std::io::Error>;
}

#[derive(Debug)]
pub struct user{
    pub pseudo : String,
    pub mail : String,
    pub mdp : String,
}

impl User for user{
    fn new(name :&str , email :&str, mdp : &str) -> user{

        user{
            pseudo : name.to_string(),
            mail : email.to_string(),
            mdp : mdp.to_string(),
        }
    }
    fn sign_in(pseudo : &str, mdp : &str) -> Result<bool, std::io::Error> {
        let mut pswFile = getPwdFromFile(pseudo);
        let mut mdp_hash = bcrypt::hash(mdp).expect("errreur lors du hashage de mdp ");
        Ok(pswFile.eq(&mdp_hash))
    }
}
