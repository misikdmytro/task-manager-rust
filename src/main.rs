use clap::{Arg, Command};
use commands::{add, error::AppError, list, remove};

mod commands;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Manage your tasks from the command line")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(Arg::new("title").required(true))
                .arg(Arg::new("description").required(false)),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a task by index")
                .arg(Arg::new("index").required(true)),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches
            .get_one::<String>("title")
            .ok_or_else(|| AppError::ValidationError("Title is required".to_string()))?;

        let description = matches.get_one::<String>("description").map(|d| d.as_str());

        add(title, description).await?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let index = matches
            .get_one::<String>("index")
            .and_then(|i| i.parse::<i64>().ok())
            .ok_or_else(|| AppError::ValidationError("Invalid index".to_string()))?;

        remove(&index).await?;
    } else if let Some(_) = matches.subcommand_matches("list") {
        list().await?;
    }

    Ok(())
}
