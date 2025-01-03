use crate::entity::moment;
use crate::enums::DataBaseError;
use crate::models::dto::moment_dto::MomentDTO;
use crate::models::moment::Moment;
use crate::utils::MarkdownParser;
use rbs::{to_value, value::map::ValueMap};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
pub struct MomentService;

impl MomentService {
    //获取所有的动态
    pub(crate) async fn get_moments(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let page = moment::Entity::find().paginate(db, page_size);
        let models = page.fetch_page(page_num).await?;
        let mut list: Vec<Moment> = vec![];
        for mut model in models {
            let content=MarkdownParser::parser_html(model.content);
            model.content =content;
            list.push(model.into());
        }
        let mut value_map = ValueMap::new();
        value_map.insert(to_value!("pageNum"), to_value!(page_num));
        value_map.insert(to_value!("pageSize"), to_value!(page_size));
        value_map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        value_map.insert(to_value!("total"), to_value!(page.num_items().await?));
        value_map.insert(to_value!("list"), to_value!(list));
        Ok(value_map)
    }
    //创建动态
    pub async fn create_and_update_moment(
        mut moment: MomentDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        // let tx = get_conn().await;
        // let row;
        // if moment.get_id().unwrap_or(0) > 0 {
        //     row = MomentDTO::update_by_column(&tx, &moment, "id").await?;
        // } else {
        //     moment.set_create_time(DateTime::now().to_string());
        //     row = MomentDTO::insert(&tx, &moment).await?;
        // }

        // Ok(row.rows_affected)
        todo!()
    }

    //获取公开的动态
    pub(crate) async fn get_public_moments(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        // let mut moments = MomentDao::get_public_moments(page_num)
        //     .await
        //     .unwrap_or_else(|e| {
        //         log::error!("{}", e);
        //         //出现异常则返回初始化对象
        //         Page::new(0, 0, 0, vec![])
        //     });
        // moments
        //     .records_mut()
        //     .iter_mut()
        //     .for_each(|item: &mut Moment| {
        //         item.create_time = item.create_time.as_str()[0..19].to_string();
        //         item.content = MarkdownParser::parser_html(item.content.to_owned());
        //     });
        // moments

        let page = moment::Entity::find().filter(moment::Column::IsPublished.eq(true)).paginate(db, page_size);
        let models = page.fetch_page(page_num).await?;
        let mut list: Vec<Moment> = vec![];
        for mut model in models {
            let content=MarkdownParser::parser_html(model.content);
            model.content =content;
            list.push(model.into());
        }
        let mut value_map = ValueMap::new();
        value_map.insert(to_value!("pageNum"), to_value!(page_num));
        value_map.insert(to_value!("pageSize"), to_value!(page_size));
        value_map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        value_map.insert(to_value!("total"), to_value!(page.num_items().await?));
        value_map.insert(to_value!("list"), to_value!(list));
        Ok(value_map)
    }

    /**
     * 更新动态的发布状态
     */
    pub(crate) async fn update_published(
        id: i64,
        is_published: bool,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        // let tx = get_conn().await;
        // let mut table = MomentDTO::default();
        // table.set_id(id as u16);
        // table.set_is_published(is_published);
        // let row = MomentDTO::update_by_column(&tx, &table, "id").await?;
        // Ok(row.rows_affected)
        todo!()
    }

    /**
     * 删除动态
     */
    pub(crate) async fn delete_moment(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        // let tx = get_conn().await;
        // let row = MomentDTO::delete_by_column(&tx, "id", id).await?;
        // Ok(row.rows_affected)
        todo!()
    }

    /**
     * 获取ID动态
     */

    pub(crate) async fn get_moment_by_id(id: i64, db: &DatabaseConnection) -> Option<Moment> {
        // let tx = get_conn().await;
        // let mut moments = Moment::select_by_column(&tx, "id", id)
        //     .await
        //     .unwrap_or_else(|e| {
        //         log::error!("get_moment_by_id error:{}", e);
        //         //出现异常则返回初始化对象
        //         vec![]
        //     });
        // if moments.len() > 0 {
        //     let moment = moments.pop()?;
        //     return Some(moment);
        // }
        // None
        todo!()
    }

    pub async fn moment_like(id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        // let executor = get_conn().await;
        // let mut table = Moment::select_by_column(&executor, "id", id.to_string().as_str()).await?;
        // for item in table.iter_mut() {
        //     item.likes += 1;
        //     let query = MomentDao::update_likes(item.id.unwrap_or_default(), item.likes).await?;
        //     return Ok(query);
        // }
        // Ok(0)
        todo!()
    }
}
