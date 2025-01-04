//use crate::config::Config;
//use deadpool_redis::Pool;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub(crate) mysql_connection: DatabaseConnection,
    // pub(crate) redis_connection: Pool,

    // pub(crate) config: Config,
}

impl AppState {
    pub fn new(
        mysql_connection: DatabaseConnection,
        // redis_connection: Pool,
        // config: Config,
    ) -> Self {
        Self {
            mysql_connection,
            // redis_connection,
            // config,
        }
    }

    pub fn get_mysql_pool(&self) -> &DatabaseConnection {
        &self.mysql_connection
    }

    // pub fn get_redis_pool(&self) -> &Pool {
    //     &self.redis_connection
    // }

    // pub fn get_config(&self) -> &Config {
    //     &self.config
    // }
}

pub async fn get_connection() -> DatabaseConnection {
    let opt = ConnectOptions::new("mysql://root:root@127.0.0.1:3306/zero_api")
        .max_connections(100)
        .min_connections(10)
        .sqlx_logging(false)
        .to_owned();
    Database::connect(opt)
        .await
        .expect("Failed to connect to database")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn test_get_connection() {
        let conn = get_connection().await;
        assert!(conn.ping().await.is_ok());
    }
}
