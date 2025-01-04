use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Series {
    id: i64,
    name: String,
    value: u64,
}

impl Series {
    pub fn new(id: i64, name: String, value: u64) -> Self {
        Self { id, name, value }
    }
}
