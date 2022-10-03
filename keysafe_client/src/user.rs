pub trait User{
    fn new(pseudo: &str, mail: &str, mdp : &str) -> user;
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
}
