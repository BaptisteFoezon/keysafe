pub trait Login{
    fn new(url: &str, mail: &str, pwd : &str) -> login;
}

#[derive(Debug)]
pub struct user{
    pub url : String,
    pub mail : String,
    pub pwd : String,
}

impl User for user{
    fn new(url :&str , mail :&str, password : &str) -> login{
        login{
            url : url.to_string(),
            mail : mail.to_string(),
            pwd : pwd.to_string(),
        }
    }
}