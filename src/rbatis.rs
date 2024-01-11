use rbatis::rbatis::Rbatis;
use crate::config;

//初始化rbatis并连接
pub async fn init_rbatis() -> Rbatis {
    let conf =config::default();
    let rbatis = Rbatis::new();
    // 这里需要你的数据库 URL
    let db_url = format!("mysql://{}:{}@{}/{}",conf.mysql.user_name,conf.mysql.password,conf.mysql.address,conf.mysql.data_base);
    rbatis.link(&*db_url).await.expect("数据库连接失败");
    rbatis
}

