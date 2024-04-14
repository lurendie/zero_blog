use crate::models::vo::{blog_detail::BlogDetail, blog_info::BlogInfo};
use crate::models::{category::Category, tag::Tag};
use crate::rbatis::RBATIS;
use rbatis::rbdc::DateTime;
use rbatis::{Error, Page, PageRequest};
use rbs::to_value;
use serde::{Deserialize, Serialize};

// 获取公开的博文
pub async fn get_blog_list_by_is_published(
    page_num: u64,
    page_size: u64,
) -> Result<Page<BlogInfo>, rbatis::Error> {
    let mut page = BlogInfo::select_page(
        &RBATIS.acquire().await.expect("异常"),
        &PageRequest::new(page_num, page_size),
    )
    .await
    .unwrap_or_else(|e: rbatis::Error| {
        log::error!("{e}",);
        Page::new(0, 0)
    });
    Ok(page)
}

pub async fn get_blog_list() -> Result<Vec<BlogInfo>, rbatis::Error> {
    let sql = "
        select
             blog.id,blog.title,blog.description,blog.create_time,blog.views ,blog.words,blog.read_time,blog.password,blog.is_top
         from
             blog  WHERE is_published = ?";
    let blog_info = RBATIS
        .query_decode::<Vec<BlogInfo>>(&sql, vec![to_value!(true)])
        .await
        .unwrap_or_else(|e| {
            log::error!("{e}");
            vec![]
        });
    
    Ok(blog_info)
}

//根据名称查询该分类博文
pub async fn get_by_category(
    name: String,
    page_num: usize,
    page_size: u64,
) -> Result<Page<BlogInfo>, rbatis::Error> {
    //分类查询
    let sql = format!(
        "
    select
     id,category_name as name
    from
     category
     where category.category_name = \"{}\"
 ",
        name
    );
    let args = vec![];
    let category_query = RBATIS.query_decode::<Category>(&*sql, args).await;
    let category = category_query.unwrap_or_else(|e| {
        log::error!("{e}");
        Category::default()
    });
    //博文查询
    let page_request = PageRequest::new(page_num.try_into().unwrap(), page_size);
    let page = BlogInfo::select_page_by_categoryid(
        &RBATIS.acquire().await.expect("异常"),
        &page_request,
        category.id.to_string().as_str(),
    )
    .await
    .unwrap_or_else(|e| {
        log::error!("异常:{e}");
        Page::new(0, 0)
    });
    Ok(page)
}

pub(crate) async fn get_by_id(id: u16) -> Option<BlogDetail> {
    let blog_detail =
        BlogDetail::select_by_id(&RBATIS.acquire().await.unwrap(), id.to_string()).await;
    blog_detail.unwrap_or_else(|e| {
        log::error!("{e}");
        None
    })
}

//根据标签名称查询该分类博文
pub async fn get_by_tag(
    name: String,
    page_num: usize,
    page_size: u64,
) -> Result<Page<BlogInfo>, rbatis::Error> {
    let sql = format!(
        "
    select
    id,tag_name as name,color
    from
    tag
     where tag_name = \"{}\"
 ",
        name
    );
    let args = vec![];
    let tag_query = RBATIS.query_decode::<Tag>(&*sql, args).await;
    let tag = tag_query.unwrap_or_else(|e| {
        println!("{:?}", e);
        Tag {
            id: Some(0),
            name: "未知".to_string(),
            color: "#000000".to_string(),
        }
    });
    let sql = "select 
    blog.*
    from blog,blog_tag where blog.id=blog_tag.blog_id and blog_tag.tag_id= ? limit ?,?";
    let args = vec![
        to_value!(tag.id.unwrap()),
        to_value!((page_num as u64 - 1) * page_size),
        to_value!(page_size),
    ];
    let blog_query = RBATIS.query_decode::<Vec<BlogInfo>>(&sql, args).await;

    let page = Page {
        records: blog_query.unwrap_or_else(|e| {
            log::error!("{:?}", e);
            vec![]
        }),
        total: 7,
        page_no: page_num.try_into().unwrap(),
        page_size,
        do_count: true,
    };
    Ok(page)
}

//博文日期时间映射结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BlogDateTime {
    pub(crate) create_time: DateTime,
}

//查询所有的公开文章的日期时间
pub(crate) async fn get_all_datetime() -> Result<Vec<BlogDateTime>, Error> {
    let sql = "select 
    blog.create_time
    from blog where is_published = ? GROUP BY create_time ORDER BY create_time DESC";
    let datetime_query = RBATIS
        .query_decode::<Vec<BlogDateTime>>(sql, vec![to_value!(1)])
        .await
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            vec![]
        });
    Ok(datetime_query)
}

//根据时间查询博文
pub(crate) async fn get_by_date(date_time: String) -> Result<Vec<BlogInfo>, Error> {
    let year = &date_time.as_str()[0..4];
    let month = &date_time.as_str()[6..];
    let sql = "SELECT *
        FROM blog
        WHERE YEAR(create_time) = ?
          AND MONTH(create_time) = ?;";
    let datetime_query = RBATIS
        .query_decode::<Vec<BlogInfo>>(sql, vec![to_value!(year), to_value!(month)])
        .await
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            vec![]
        });
    Ok(datetime_query)
}

//查询公开文章的数量
pub(crate) async fn get_archives_count() -> Result<u64, Error> {
    let sql = "SELECT count(id)
    FROM blog
    WHERE is_published = 1";
    let count = RBATIS
        .query_decode::<u64>(sql, vec![])
        .await
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            0
        });
    Ok(count)
}

#[cfg(test)]
mod test {
    use rbatis::rbdc::DateTime;
    use rbatis::Page;
    //使用Rbatis Page结构体测试自定义数据
    #[test]
    fn test_my_page() {
        let arr = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let page = Page {
            records: arr,
            total: 5 as u64,
            page_no: 1,
            page_size: 5,
            do_count: true,
        };
        println!("{:?}", page)
    }

    #[test]
    fn test_format() {
        let date = DateTime::now().format("YYYY-MM-DD");
        println!("{}", date);
        let mut date_time = String::from("2021-12");
        date_time.insert(4, '年');
        date_time.insert(10, '日');
        println!("{}", date_time);
    }
}
