use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    id: u16,
   // #[serde(rename(deserialize = "category_name"))]
    name: String,
}

impl Category{
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }

    
    pub fn get_id(&self) -> u16 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    

    pub fn set_id(&mut self, id: u16) { 
        self.id = id;
    }
}

crud!(Category {});
