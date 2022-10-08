pub enum UserTypes {
    User,
    Admin
}

pub struct User {
    pub token: String,
    pub usertype: UserTypes
}
