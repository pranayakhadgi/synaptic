use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use colored::*;
use db::{Database, TaskStatus};

mod db;

#[derive(Parser)]
#[command(name = "synaptic")]
#[command(about = "Personal cognitive operating system")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        title: String,
        /// Due date (YYYY-MM-DD)
        #[arg(short, long)]
        due: Option<NaiveDate>,
        /// Tags (can use multiple times)
        #[arg(short, long)]
        tag: Vec<String>,
    },
    /// List all tasks
    List {
        /// Show completed tasks too
        #[arg(short, long)]
        all: bool,
    },
    /// Mark task as complete
    Done {
        /// Task ID
        id: u64,
    },
    /// Remove a task permanently
    Remove {
        /// Task ID
        id: u64,
    },
}

fn main() {
    let db = Database::open("synaptic.db").expect("Failed to open database");
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { title, due, tag } => {
            let id = db.add_task(&title, due, tag.clone())
                .expect("Failed to add task");
            
            let due_str = due.map(|d| d.to_string())
                .unwrap_or_else(|| "no due date".to_string());
            
            let tag_str = if tag.is_empty() {
                "no tags".to_string()
            } else {
                format!("tags: {}", tag.join(", "))
            };
            
            println!("{} {} {} (Due: {}, {})", 
                "✓".green().bold(),
                "Created task".green(),
                id.to_string().cyan(),
                due_str.yellow(),
                tag_str.dimmed()
            );
        }
        
        Commands::List { all } => {
            let tasks = db.list_tasks(all).expect("Failed to list tasks");
            
            if tasks.is_empty() {
                println!("{}", "No tasks found. Your mind is clear.".dimmed());
                return;
            }
            
            println!("{:<4} {:<10} {:<20} {:<12} {}", 
                "ID".bold(), "STATUS".bold(), "TITLE".bold(), "DUE".bold(), "TAGS".bold()
            );
            println!("{}", "─".repeat(60).dimmed());
            
            for task in tasks {
                let status_icon = if task.status == TaskStatus::Done {
                    "✓".green()
                } else {
                    "○".dimmed()
                };
                
                let status_text = if task.status == TaskStatus::Done {
                    "done".green()
                } else {
                    "todo".white()
                };
                
                let due_str = match task.due_date {
                    Some(d) => {
                        let today = chrono::Utc::now().date_naive();
                        let due_naive = d.date_naive();
                        
                        if due_naive < today {
                            d.format("%Y-%m-%d").to_string().red().to_string()
                        } else if due_naive == today {
                            "TODAY".yellow().to_string()
                        } else {
                            d.format("%Y-%m-%d").to_string().dimmed().to_string()
                        }
                    }
                    None => "-".dimmed().to_string(),
                };
                
                let tags_str = if task.tags.is_empty() {
                    "".to_string()
                } else {
                    task.tags.iter()
                        .map(|t| format_tag(t))
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                
                let title = if task.status == TaskStatus::Done {
                    task.title.dimmed().to_string()
                } else {
                    task.title.white().to_string()
                };
                
                println!("{:<4} {:<10} {:<20} {:<12} {}", 
                    task.id.to_string().dimmed(),
                    format!("{} {}", status_icon, status_text),
                    truncate(&title, 20),
                    due_str,
                    tags_str
                );
            }
        }
        
        Commands::Done { id } => {
            match db.complete_task(id) {
                Ok(true) => println!("{} Task {} marked as {}", 
                    "✓".green(), 
                    id.to_string().cyan(),
                    "done".green()
                ),
                Ok(false) => println!("{} Task {} not found", 
                    "✗".red(), 
                    id
                ),
                Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
            }
        }
        
        Commands::Remove { id } => {
            match db.delete_task(id) {
                Ok(true) => println!("{} Task {} {}", 
                    "🗑".red(), 
                    id.to_string().cyan(),
                    "deleted".red()
                ),
                Ok(false) => println!("{} Task {} not found", 
                    "✗".red(), 
                    id
                ),
                Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
            }
        }
    }
}

fn format_tag(tag: &str) -> String {
    let colored = match tag.as_str() {
        "work" => tag.yellow(),
        "personal" => tag.blue(),
        "urgent" => tag.red().bold(),
        _ => tag.dimmed(),
    };
    format!("[{}]", colored)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}…", &s[..max_len-1])
    } else {
        s.to_string()
    }
}
