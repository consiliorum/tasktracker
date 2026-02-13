mod db;
mod export;
mod models;

use clap::{Parser, Subcommand};
use colored::Colorize;
use comfy_table::{Table, ContentArrangement};
use models::Priority;

#[derive(Parser)]
#[command(name = "tasktracker", about = "A CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        /// Description
        #[arg(short = 'd', long = "desc")]
        description: Option<String>,
        /// Priority (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: Priority,
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,
    },
    /// List tasks
    List {
        /// Show all tasks including completed
        #[arg(long)]
        all: bool,
        /// Show only completed tasks
        #[arg(long)]
        done: bool,
        /// Sort by: priority or due
        #[arg(long, default_value = "priority")]
        sort: String,
    },
    /// Mark a task as done
    Done {
        /// Task ID
        id: i64,
    },
    /// Remove a task
    Remove {
        /// Task ID
        id: i64,
    },
    /// Edit an existing task
    Edit {
        /// Task ID
        id: i64,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// New description
        #[arg(long = "desc")]
        description: Option<String>,
        /// New priority
        #[arg(short, long)]
        priority: Option<Priority>,
        /// New due date
        #[arg(long)]
        due: Option<String>,
    },
    /// Export tasks to Markdown
    Export {
        /// Output file (defaults to stdout)
        #[arg(long)]
        file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let conn = db::open().expect("Failed to open database");

    match cli.command {
        Commands::Add { title, description, priority, due } => {
            let id = db::add_task(
                &conn,
                &title,
                description.as_deref(),
                &priority,
                due.as_deref(),
            )
            .expect("Failed to add task");
            println!("{} Task #{} added: {}", "✓".green(), id, title);
        }
        Commands::List { all, done, sort } => {
            let mut tasks = db::list_tasks(&conn, all || done).expect("Failed to list tasks");
            if done && !all {
                tasks.retain(|t| t.done);
            }
            match sort.as_str() {
                "priority" => tasks.sort_by_key(|t| t.priority.sort_order()),
                "due" => tasks.sort_by(|a, b| {
                    let da = a.due_date.as_deref().unwrap_or("9999-99-99");
                    let db = b.due_date.as_deref().unwrap_or("9999-99-99");
                    da.cmp(db)
                }),
                _ => {}
            }

            if tasks.is_empty() {
                println!("No tasks found.");
                return;
            }

            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(vec!["ID", "Title", "Priority", "Due", "Status"]);

            for task in &tasks {
                let priority_str = match task.priority {
                    Priority::High => "high".red().bold().to_string(),
                    Priority::Medium => "medium".yellow().to_string(),
                    Priority::Low => "low".green().to_string(),
                };
                let status = if task.done {
                    "done".dimmed().to_string()
                } else {
                    "pending".white().to_string()
                };
                let due = task.due_date.as_deref().unwrap_or("-").to_string();
                table.add_row(vec![
                    task.id.to_string(),
                    task.title.clone(),
                    priority_str,
                    due,
                    status,
                ]);
            }
            println!("{table}");
        }
        Commands::Done { id } => {
            if db::mark_done(&conn, id).expect("Failed to update task") {
                println!("{} Task #{} marked as done.", "✓".green(), id);
            } else {
                eprintln!("{} Task #{} not found.", "✗".red(), id);
                std::process::exit(1);
            }
        }
        Commands::Remove { id } => {
            if db::remove_task(&conn, id).expect("Failed to remove task") {
                println!("{} Task #{} removed.", "✓".green(), id);
            } else {
                eprintln!("{} Task #{} not found.", "✗".red(), id);
                std::process::exit(1);
            }
        }
        Commands::Edit { id, title, description, priority, due } => {
            if title.is_none() && description.is_none() && priority.is_none() && due.is_none() {
                eprintln!("Nothing to edit. Provide at least one of --title, --desc, --priority, --due.");
                std::process::exit(1);
            }
            let updated = db::edit_task(
                &conn,
                id,
                title.as_deref(),
                description.as_deref(),
                priority.as_ref(),
                due.as_deref(),
            )
            .expect("Failed to edit task");
            if updated {
                println!("{} Task #{} updated.", "✓".green(), id);
            } else {
                eprintln!("{} Task #{} not found.", "✗".red(), id);
                std::process::exit(1);
            }
        }
        Commands::Export { file } => {
            let tasks = db::list_tasks(&conn, true).expect("Failed to list tasks");
            let md = export::to_markdown(&tasks);
            if let Some(path) = file {
                std::fs::write(&path, &md).expect("Failed to write file");
                println!("{} Exported to {}", "✓".green(), path);
            } else {
                print!("{}", md);
            }
        }
    }
}
