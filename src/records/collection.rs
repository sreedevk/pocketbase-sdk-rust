use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Resource {
}

pub struct Collection<T> {
    page: u32,
    per_page: u32,
    total_items: u32,
    items: Vec<T>,
}
