use crate::file_manager::{FileManager, FileManagerTrait};
use pwhash::bcrypt;

pub trait BouncerTrait {
    fn new() -> Bouncer;
    fn sign_in(&self, pseudo: &str, pwd: &str) -> Result<bool, std::io::Error>;
}

#[derive(Debug)]
pub struct Bouncer {}

impl BouncerTrait for Bouncer {
    fn new() -> Bouncer {
        Bouncer {}
    }
    fn sign_in(&self, pseudo: &str, pwd: &str) -> Result<bool, std::io::Error> {
        let psw_file = FileManager::get_pwd_from_file(pseudo)?;
        Ok(bcrypt::verify(pwd, &*psw_file))
    }
}
