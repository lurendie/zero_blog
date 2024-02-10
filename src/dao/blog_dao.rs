use rbatis::executor::Executor;
use rbatis::{Page, PageRequest};
use rbs::to_value;
use rbson::Bson;
use crate::models::{category::Category,tag::Tag};
use crate::models::vo::blog_info::BlogInfo;
use crate::rbatis::RBATIS;

    pub async fn get_blog_list_by_is_published(page_num:u64,page_size:u64) -> Result<Page<BlogInfo>,rbatis::Error>{
        let _sql = r#"
        select
             blog.id as "id",blog.title as "title",blog.description as "description",blog.create_time as "create_time",blog.views as "views",blog.words as "words",blog.read_time as "read_time",blog.password as "password",blog.is_top as "is_top"
         from
             blog  WHERE is_published = ?"#;
        let mut page =BlogInfo::select_page(&RBATIS.acquire().await.expect("异常"), &PageRequest::new(page_num, page_size)).await.unwrap();
        for item in page.records.iter_mut(){
            let id =item.id.unwrap();
            let sql = format!("
               select
                id,category_name as name
               from
	            category
	            where category.id =(select category_id  from blog where blog.id ={})
            ",id);
            let args = vec![];
            let category = RBATIS.query_decode::<Category>(&sql, args).await.expect("查询分类异常");
            item.category=Some(category);
            if item.password !=""{
                item.private =Some(true);
            }else{
                item.private =Some(false)
            }
            item.password =String::from("");
            //转HTML
            item.description=markdown::to_html(&item.description);
            //TagList
            let sql = format!("
            select
            tag.id as id,tag_name as name,color
            from blog_tag,tag
            where blog_tag.tag_id = tag.id and blog_tag.blog_id = {}
            ",id);
            let tags=RBATIS.query_decode::<Vec<Tag>>(&*sql, vec![]).await.expect("查询标签异常");
            item.tags = Some(tags);
        }
        Ok(page)
    }

    pub async fn get_blog_list()->Result<Vec<BlogInfo>,rbatis::Error>{
        let sql = "
        select
             blog.id,blog.title,blog.description,blog.create_time,blog.views ,blog.words,blog.read_time,blog.password,blog.is_top
         from
             blog  WHERE is_published = ?";
        let mut blog_info =RBATIS.query_decode::<Vec<BlogInfo>>(&sql, vec![to_value!(1)]).await.expect("异常");
        for  item in blog_info.iter_mut(){
            let id =item.id.unwrap();
            let sql = format!("
            select
             id,category_name as name
            from
             category
             where category.id =(select category_id  from blog where blog.id ={})
         ",id);
            let args = vec![];
            let category = RBATIS.query_decode::<Category>(&sql, args).await;
            item.category=Some(category.expect("异常"));
            if item.password !=""{
                item.private =Some(true);
            }else{
                item.private =Some(false)
            }
            item.password =String::from("");
            //转HTML
            item.description=markdown::to_html(&item.description);
            //TagList
            let sql = format!("
            select
            tag.id as id,tag_name as name,color
            from blog_tag,tag
            where blog_tag.tag_id = tag.id and blog_tag.blog_id = {}
            ",id);
            let tags=RBATIS.query_decode::<Vec<Tag>>(&*sql, vec![]).await.expect("查询标签异常");
            item.tags = Some(tags);
        }
        Ok(blog_info)     
    }

//根据名称查询该分类博文
pub async fn get_by_name(name :String,page_num:usize,page_size:u64) ->Result<Page<BlogInfo>,rbatis::Error>{
    //分类查询
    let sql = format!("
    select
     id,category_name as name
    from
     category
     where category.category_name = \"{}\"
 ",name);
    let args = vec![];
    let category = RBATIS.query_decode::<Category>(&*sql, args).await.expect("查询分类异常");
    //博文查询
    let page_request = PageRequest::new(page_num.try_into().unwrap(), page_size);
    let sql = r#"
    select
         blog.id as "id",blog.title as "title",blog.description as "description",blog.create_time as "create_time",blog.views as "views",blog.words as "words",blog.read_time as "read_time",blog.password as "password",blog.is_top as "is_top"
     from
         blog  WHERE is_published = ? and category_id = ?"#;
    let args = vec![Bson::from(1),Bson::from(category.id as u32)];
    let mut page =BlogInfo::select_page(&RBATIS.acquire().await.expect("异常"), &page_request).await.unwrap();
    for  item in page.records.iter_mut(){
        let id =item.id.unwrap();
        let sql = format!("
           select
            id,category_name as name
           from
            category
            where category.id =(select category_id  from blog where blog.id =\"{}\")
        ",id);
        let args = vec![];
        let category = RBATIS.query_decode::<Category>(&*sql, args).await.expect("查询分类异常");
        item.category=Some(category);
        if item.password !=""{
            item.private =Some(true);
        }else{
            item.private =Some(false)
        }
        item.password =String::from("");
        //转HTML
        item.description=markdown::to_html(&item.description);
        //TagList
        let sql = format!("
        select
        tag.id as id,tag_name as name,color
        from blog_tag,tag
        where blog_tag.tag_id = tag.id and blog_tag.blog_id = \"{}\"
        ",id);
        let tags=RBATIS.query_decode::<Vec<Tag>>(&*sql, vec![]).await.expect("查询标签异常");
        item.tags = Some(tags);
    }
    Ok(page)
}    