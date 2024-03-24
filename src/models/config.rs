use serde::Deserialize;

use super::{
    fqdn::Fqdn,
    user::User,
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Config {
    pub users: Vec<User>,
    pub fqdns: Vec<Fqdn>
}

impl Config{
    pub fn default() -> Self{
        Self{
            users: Vec::new(),
            fqdns: Vec::new(),
        }
    }

    pub fn has_user(&self, user: &User) -> bool{
        for one_user in self.users.as_slice(){
            if one_user.active && one_user.username == user.username &&
                    one_user.password == user.password{
                return true;
            }
        }
        false
    }

    pub fn has_fqdn(&self, addr: &str) -> bool{
        for fqdn in self.fqdns.as_slice(){
            if fqdn.active && fqdn.addr.to_lowercase() == addr.to_lowercase(){
                return true;
            }
        }
        false
    }
}
