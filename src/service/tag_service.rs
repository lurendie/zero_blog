use crate::models::tag::Tag;
use rbatis::Error;
use crate::dao::tag_dao::get_list;

pub async fn get_tags()->Result<Vec<Tag>,Error>{
    get_list().await
}