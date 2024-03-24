use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Fqdn {
    pub addr: String,
    pub active: bool,
}

impl fmt::Display for Fqdn{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} => {}", self.addr, self.active)
    }
}
