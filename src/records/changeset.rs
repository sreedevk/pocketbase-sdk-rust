use serde::Serialize;
use crate::users::User;

#[derive(Serialize)]
pub struct Changeset<'a, T> {
    pub user:  &'a User<'a>,
    pub resource: &'a str,
    pub record: &'a T
}
