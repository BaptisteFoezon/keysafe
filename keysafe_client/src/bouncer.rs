use crate::file_manager::get_pwd_from_file;
use pwhash::bcrypt;

pub trait Bouncer {
    fn new() -> bouncer;
    fn sign_in(&self, pseudo: &str, pwd: &str) -> Result<bool, std::io::Error>;
}

#[derive(Debug)]
pub struct bouncer {}

impl Bouncer for bouncer {
    fn new() -> bouncer {
        bouncer {}
    }
    fn sign_in(&self, pseudo: &str, pwd: &str) -> Result<bool, std::io::Error> {
        let psw_file = get_pwd_from_file(pseudo);
        Ok(bcrypt::verify(pwd, &*psw_file))
    }
}
