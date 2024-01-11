use async_trait::async_trait;
use rbatis::{IPageRequest, Page, PageRequest};
use rbatis::crud::CRUD;
use rbson::Bson;
use crate::dao::blog_dao::BlogDao;
use crate::models::category::Category;
use crate::models::vo::blog_info::BlogInfo;
use crate::rbatis::init_rbatis;


pub struct BlogDaoImpl;

#[async_trait]
impl BlogDao for BlogDaoImpl{
    async fn get_blog_list_by_is_published(pageNum:u64,page_size:u64) -> Result<Page<BlogInfo>,rbatis::Error>{
        let rbatis =init_rbatis().await;
        let page_request = PageRequest::new(pageNum, page_size);
        let sql = r#"
        select
             blog.id as "id",blog.title as "title",blog.description as "description",blog.create_time as "create_time",blog.views as "views",blog.words as "words",blog.read_time as "read_time",blog.password as "password",blog.is_top as "is_top"
         from
             blog  WHERE is_published = ?"#;
        let args = vec![Bson::from(1)];
        let mut page =rbatis.fetch_page::<BlogInfo>(&sql, args, &page_request).await.unwrap();
        for  item in page.records.iter_mut(){
            let id =item.id.unwrap();
            let sql = format!("
               select
                category.*
               from
	            category
	            where category.id =(select category_id  from blog where blog.id ={})
            ",id);
            let args = vec![];
            let category = rbatis.fetch::<Category>(&*sql, args).await.expect("查询分类异常");
            item.category=Some(category);
            if item.password !=""{
                item.private =Some(true);
            }else{
                item.private =Some(false)
            }
            item.password =String::from("");
        }
        Ok(page)
    }

    // async fn get_blog_list_by_is_published(pageNum:u64,page_size:u64) -> Page<BlogInfo>{
    //     let rbatis =init_rbatis().await;
    //     let page_request = PageRequest::new(pageNum, page_size);
    //     let sql = r#"
    //     select
    //         blog.id, blog.title, blog.description, blog.create_time, blog.views, blog.words, blog.read_time, blog.password, blog.is_top, blog.is_published,
    //         category.id as "category_id", category.category_name as "category_name"
    //     from
    //         blog
    //     LEFT JOIN category ON blog.category_id = category.id
    //     WHERE
    //         blog.is_published = ?
    // "#;
    //     let args = vec![Bson::from(1)];
    //     let mut page =rbatis.fetch_page::<BlogInfo>(&sql, args, &page_request).await.unwrap();
    //     page
    // }
}
