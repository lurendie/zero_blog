use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: u16,
    pub name: String,
}

crud!(Category {});
