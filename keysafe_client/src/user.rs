pub trait User{
    fn new(name: String, email: String, mdp : String) -> user;

}

#[derive(Debug)]
pub struct user{
    pub name : String,
    pub email : String,
    pub mdp : String,

}

impl User for user{
    fn new(name :String , email :String, mdp : String) -> user{
        user{
            name,
            email,
            mdp
        }
    }

}
