use crate::models::category::Category;
use crate::rbatis::RBATIS;

//查询所有分类
pub async fn get_list() ->Vec<Category>{
    let sql ="select id,category_name as name from category";
   let category:Vec<Category> =match RBATIS.fetch(sql,Vec::new()).await{
        Ok(ok)=>{
            ok
        },
       Err(e)=>panic!("{}",e),
    };
    category
}
