pub trait Bouncer{
    fn new() -> bouncer;
    fn sign_in(&self) -> Result<bool, std::io::Error>;
}

#[derive(Debug)]
pub struct bouncer{
}

impl Bouncer for bouncer{
    fn new() -> bouncer{
        bouncer{
        }
    }
    fn sign_in(&self) -> Result<bool, std::io::Error>{
        return Ok(true);
    }
}