pub trait Bouncer{
    fn new() -> bouncer;
    fn sign_in(&self, pseudo :&str , pwd : &str) -> Result<bool, std::io::Error>;
}

#[derive(Debug)]
pub struct bouncer{
}

impl Bouncer for bouncer{
    fn new() -> bouncer{
        bouncer{
        }
    }
    fn sign_in(&self, pseudo : &str , pwd : &str) -> Result<bool, std::io::Error>{
        return if pwd.to_string().eq("a") {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}