//Blog
pub struct Blog{
    id :u16,
    title:String,
    first_picture :String,
    content :String,
    description :String,
    is_published:bool,
    is_recommend:bool,
    is_appreciation:bool,
    is_comment_enabled:bool,
    create_time:chrono::NaiveDateTime,
    update_time:chrono::NaiveDateTime,
    views:String,
    words:String,
    read_time:String,
    category_id:u16,
    is_top:bool,
    password:String,
    user_id:u16
}