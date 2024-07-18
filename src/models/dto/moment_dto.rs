use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentDTO {
    id: Option<u64>,
    pub(crate) content: String,
    #[serde(rename(deserialize = "createTime"))]
    pub(crate) create_time: String,
    likes: u64,
    #[serde(rename(deserialize = "published"))]
    is_published: u8,
}
crud!(MomentDTO {}, "moment");
impl_select_page!(MomentDTO{select_page() => "`where is_published = 1`"});
