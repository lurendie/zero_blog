use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;
use crate::config::Config;

// 定义一个静态变量来存储Rbatis连接实例。
// 定义一个全局的 RBatis 实例，使用 Lazy 来保证线程安全的单次初始化
pub static RBATIS: Lazy<Rbatis> = Lazy::new(|| {
    let rbatis = Rbatis::new();
    rbatis
});

pub(crate) async fn init_rbatis(conf: &Config) -> Result<(), rbatis::Error> {
    let db_url = format!(
        "mysql://{}:{}@{}/{}",
        conf.mysql.user_name,
        conf.mysql.password,
        conf.mysql.address,
        conf.mysql.data_base
    );
    RBATIS.link(&db_url).await.expect("TODO: panic message");
    log::debug!("数据库连接初始化完成！");
    Ok(())
}