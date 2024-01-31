use rbatis::crud_table;

#[crud_table]
#[derive(Debug, Clone)]
pub struct BlogTag{
    blog_id:Option<u16>,
    tag_id:u16
}