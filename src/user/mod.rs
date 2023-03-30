#[derive(Debug)]
pub enum UserTypes {
    User,
    Admin,
}

#[derive(Debug)]
pub struct User {
    pub token: String,
    pub usertype: UserTypes,
}
