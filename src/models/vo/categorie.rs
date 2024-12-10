use rbatis::impl_select_page;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Categorie {
    id: Option<u16>,
    #[serde(rename(deserialize = "category_name"))]
    name: String,
    #[serde(skip)]
    _blogs: Vec<Categorie>,
}

impl_select_page!(Categorie{select_page()=>""},"category");
impl Categorie {
    pub fn new(id: Option<u16>, name: String, blogs: Vec<Categorie>) -> Categorie {
        Categorie { id, name, _blogs: blogs }
    }
}
