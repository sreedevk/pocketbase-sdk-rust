use crate::{client::Client, records::Changeset};

pub async fn create<T: Changeset>(collection: String, changeset: &T, client: &Client) {
}
