pub mod operations;
use std::collections::HashMap;

pub trait Recordable {
    fn new(fields: HashMap<String, String>) -> Self;
}
