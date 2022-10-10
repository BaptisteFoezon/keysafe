pub trait Login {
    fn new(url: &str, mail: &str, pwd: &str) -> login;
}

#[derive(Debug)]
pub struct login {
    pub url: String,
    pub mail: String,
    pub pwd: String,
}

impl Login for login {
    fn new(url: &str, mail: &str, pwd: &str) -> login {
        login {
            url: url.to_string(),
            mail: mail.to_string(),
            pwd: pwd.to_string(),
        }
    }
}