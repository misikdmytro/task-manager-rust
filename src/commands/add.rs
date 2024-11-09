use crate::commands::{conn::connect, error::AppError};

pub async fn add(title: &str, description: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // check len between 1 and 255
    if title.is_empty() || title.len() > 255 {
        return Err(Box::new(AppError::ValidationError("Title must be between 1 and 255 characters".to_string())));
    }

    let pool = connect().await?;

    sqlx::query("INSERT INTO tasks (title, description) VALUES (?, ?)")
        .bind(title)
        .bind(description)
        .execute(&pool)
        .await?;

    println!("Task '{}' added!", title);

    Ok(())
}