pub trait User{
    fn new(name: &str, email: &str, mdp : &str) -> user;
}

#[derive(Debug)]
pub struct user{
    pub name : String,
    pub email : String,
    pub mdp : String,
}

impl User for user{
    fn new(name :&str , email :&str, mdp : &str) -> user{
        user{
            name : name.to_string(),
            email : email.to_string(),
            mdp : mdp.to_string(),
        }
    }
}
