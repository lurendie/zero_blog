use rbatis::Page;

use crate::{dao::moment_dao, models::moment::Moment};
//获取公开的动态
pub(crate) async fn get_moments(page_num:usize)->Page<Moment>{
    let moments=moment_dao::get_moments(page_num).await
    .unwrap_or_else(|e|{
        log::error!("{}",e);
        //出现异常则返回初始化对象
        Page::new(0, 0)
    });
    //todo 待进行业务处理转MD文档
    moments
}