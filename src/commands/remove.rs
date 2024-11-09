use super::{conn::connect, error::AppError};

pub async fn remove(ind: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect().await?;
    let res = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(ind)
        .execute(&pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(Box::new(AppError::NotFound));
    } else {
        println!("Task {} removed!", ind);
    }

    Ok(())
}