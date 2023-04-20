use crate::client::Client;
use crate::types::FieldType;
use anyhow::Result;

pub struct Schema {
    system: bool,
    id: String,
    name: String,
    field_type: FieldType,
}

pub struct Collection {
    pub name: String,
}

impl Collection {
    fn new(client: &Client, name: String) -> Result<Self> {
        Ok(Self { name })
    }

    fn list(client: &Client) -> Result<Vec<Self>> {
        Ok(vec![])
    }
}
