use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub active: bool,
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} => {}", self.username, self.password)
    }
}
