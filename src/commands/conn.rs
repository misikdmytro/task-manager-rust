use sqlx::{Error, MySql, MySqlPool, Pool};

pub async fn connect() -> Result<Pool<MySql>, Error> {
    MySqlPool::connect("mysql://root:root@localhost:3306/task_manager").await
}