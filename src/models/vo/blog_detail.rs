use crate::models::category::Category;

//博客详情信息
struct _BlogDetail{
    id :Option<u16>,
    title:String,
    content :String,
    is_appreciation:u8,
    is_comment_enabled:u8,
    create_time:chrono::NaiveDateTime,
    update_time:chrono::NaiveDateTime,
    views:u16,
    words:u16,
    read_time:u16,
    is_top:u8,
    // todo 结构体引用
    category:Category,
    password:String,

}