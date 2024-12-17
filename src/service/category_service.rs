/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-19 23:46:51
 * @FilePath: \zero_blog\src\service\category_service.rs
 */

use rbatis::{Page, PageRequest};
use rbs::value::map::ValueMap;
use rbs::{to_value, Value};

use crate::constant::redis_key_constants;
use crate::dao::{BlogDao, CategoryDao};
use crate::models::category::Category;
use crate::models::vo::categorie::Categorie;
use crate::models::vo::serise::Series;
use crate::rbatis::get_conn;
use crate::service::RedisService;

pub struct CategoryService;

impl CategoryService {
    /**
     * 查询所有分类(首页)
     */
    pub async fn get_list() -> Vec<Value> {
        //1.查询Redis
        let result =
            RedisService::get_value_vec(redis_key_constants::CATEGORY_NAME_LIST.to_string()).await;
        if let Some(result) = result {
            let arr = match result {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        redis_key_constants::CATEGORY_NAME_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        let mut result = vec![];
        CategoryDao::get_list()
            .await
            .iter()
            .for_each(|item| result.push(to_value!(item)));

        //3.保存Redis
        RedisService::set_value_vec(
            redis_key_constants::CATEGORY_NAME_LIST.to_string(),
            &to_value!(&result),
        )
        .await;
        result
    }

    /**
     * 查询分类名称
     */
    pub async fn get_categorys_count() -> ValueMap {
        let mut map = ValueMap::new();
        let mut legend = vec![];
        let mut series = vec![];

        for item in CategoryDao::get_list().await {
            legend.push(to_value!(item.get_name()));
            let series_item = Series::new(
                item.get_id(),
                item.get_name().to_string(),
                BlogDao::get_category_count(item.get_name().to_string()).await,
            );
            series.push(series_item);
        }
        map.insert(to_value!("legend"), to_value!(legend));
        map.insert(to_value!("series"), to_value!(series));
        map
    }

    pub(crate) async fn get_categories() -> Vec<Categorie> {
        let mut list = vec![];
        CategoryDao::get_list().await.iter().for_each(|item| {
            list.push(Categorie::new(
                Some(item.get_id().clone()),
                item.get_name().to_string(),
                vec![],
            ))
        });
        list
    }

    //查询所有分类(后台)
    pub async fn get_page_categories(
        page_num: u64,
        page_size: u64,
    ) -> Result<Page<Categorie>, rbatis::rbdc::Error> {
        Categorie::select_page(&get_conn().await, &PageRequest::new(page_num, page_size)).await
    }

    pub async fn insert_category(name: String) -> Result<u64, rbatis::rbdc::Error> {
        let mut category = Category::default();
        category.set_name(name);
        CategoryDao::save_category(&category).await
    }

    pub async fn update_category(category: Category) -> Result<u64, rbatis::rbdc::Error> {
        CategoryDao::update_category(&category).await
    }

    pub async fn delete_category(id: u16) -> Result<u64, rbatis::rbdc::Error> {
        let mut category = Category::default();
        category.set_id(id);
        //判断分类是否有文章
        let count = BlogDao::get_blog_by_category_id(category.get_id()).await?;
        if count > 0 {
            return Err(rbatis::rbdc::Error::from("分类下有文章，不能删除"));
        }
        CategoryDao::delete_category(&category).await
    }
}
