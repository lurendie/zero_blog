use crate::{dao::MomentDao, models::moment::Moment};
use rbatis::{IPage, Page};
pub struct MomentService;

impl MomentService {
    //获取公开的动态
    pub(crate) async fn get_moments(page_num: usize) -> Page<Moment> {
        let mut moments = MomentDao::get_moments(page_num).await.unwrap_or_else(|e| {
            log::error!("{}", e);
            //出现异常则返回初始化对象
            Page::new(0, 0)
        });
        moments
            .get_records_mut()
            .iter_mut()
            .for_each(|item: &mut Moment| {
                item.create_time = item.create_time.as_str()[0..19].to_string();
                item.content = markdown::to_html(&item.content);
            });
        moments
    }
}
