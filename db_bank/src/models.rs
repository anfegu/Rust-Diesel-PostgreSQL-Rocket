use crate::schema::*;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    password: String,
}

impl User {
    pub fn verify_pass(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
    password: String,
}

pub fn new_user<'a>(name: &'a str, password: &str) -> NewUser<'a> {
    NewUser {
        name,
        password: bcrypt::hash(password, 7).unwrap(),
    }
}

#[derive(Queryable, Debug)]
pub struct Account {
    pub id: i32,
    pub num: String,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub num: &'a str,
    pub amount: &'a i32,
}
