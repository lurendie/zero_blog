use crate::models::tag::Tag;
use crate::rbatis::RBATIS;

pub async fn get_list()->Result<Vec<Tag>,rbatis::Error>{
    let sql = format!("
            select
            tag.id as id,tag_name as name,color
            from tag
            ");
    let tags:Result<Vec<Tag>,rbatis::Error>= RBATIS.query_decode(&sql,vec![]).await;
    tags
}