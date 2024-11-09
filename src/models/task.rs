use std::fmt::Display;

#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.done { "Done" } else { "Not done" };
        let description = self.description.as_deref().unwrap_or("None");
        write!(f, "{}. {}: {} ({})", self.id, self.title, description, status)
    }
}