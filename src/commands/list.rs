use crate::{commands::conn::connect, models::task::Task};

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect().await?;

    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await?;

    if tasks.is_empty() {
        println!("No tasks found");
    } else {
        println!("Tasks:");
        for task in tasks {
            println!("{}", task);
        }
    }

    Ok(())
}
