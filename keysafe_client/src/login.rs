pub trait LoginTrait {
    fn new(url: &str, mail: &str, pwd: &str) -> Login;
}

#[derive(Debug)]
pub struct Login {
    pub url: String,
    pub mail: String,
    pub pwd: String,
}

impl LoginTrait for Login {
    fn new(url: &str, mail: &str, pwd: &str) -> Login {
        Login {
            url: url.to_string(),
            mail: mail.to_string(),
            pwd: pwd.to_string(),
        }
    }
}
