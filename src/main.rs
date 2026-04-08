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
        title: String,
        #[arg(short, long)]
        due: Option<NaiveDate>,
        #[arg(short, long)]
        tag: Vec<String>,
    },
    /// List tasks
    List {
        #[arg(short, long)]
        all: bool,
    },
    /// Complete a task
    Done { id: u64 },
    /// Delete a task
    Remove { id: u64 },
}

fn main() {
    let db = Database::open("synaptic.db").expect("Failed to open database");
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { title, due, tag } => {
            let id = db.add_task(&title, due, tag.clone())
                .expect("Failed to add task");
            
            println!("{} Created task {}: {}", 
                "✓".green(),
                id.to_string().cyan(),
                title.bold()
            );
            
            if let Some(d) = due {
                println!("  Due: {}", d.to_string().yellow());
            }
            if !tag.is_empty() {
                println!("  Tags: {}", 
                    tag.iter().map(|t| format!("[{}]", t)).collect::<Vec<_>>().join(" ").dimmed()
                );
            }
        }
        
        Commands::List { all } => {
            let tasks = db.list_tasks(all).expect("Failed to list tasks");
            
            if tasks.is_empty() {
                println!("{}", "No tasks found. Your mind is clear.".dimmed());
                return;
            }
            
            println!("\n{:<4} {:<8} {:<25} {:<12} {}", 
                "ID".bold().underline(),
                "STATUS".bold().underline(),
                "TITLE".bold().underline(),
                "DUE".bold().underline(),
                "TAGS".bold().underline()
            );
            
            for task in tasks {
                let status_str = if task.status == TaskStatus::Done {
                    "✓ done".green()
                } else {
                    "○ todo".white()
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
                        .map(|t| match t.as_ref() {
                            "work" => format!("[{}]", t.yellow()),
                            "personal" => format!("[{}]", t.blue()),
                            "urgent" => format!("[{}]", t.red().bold()),
                            _ => format!("[{}]", t.dimmed()),
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                
                let title_display = if task.status == TaskStatus::Done {
                    task.title.dimmed().to_string()
                } else {
                    task.title.white().to_string()
                };
                
                println!("{:<4} {:<8} {:<25} {:<12} {}", 
                    task.id.to_string().dimmed(),
                    status_str,
                    truncate(&title_display, 25),
                    due_str,
                    tags_str
                );
            }
            println!();
        }
        
        Commands::Done { id } => {
            match db.complete_task(id) {
                Ok(true) => println!("{} Task {} marked as {}", 
                    "✓".green(), 
                    id.to_string().cyan(),
                    "done".green().bold()
                ),
                Ok(false) => println!("{} Task {} not found", "✗".red(), id),
                Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
            }
        }
        
        Commands::Remove { id } => {
            match db.delete_task(id) {
                Ok(true) => println!("{} Task {} deleted", "🗑".red(), id.to_string().cyan()),
                Ok(false) => println!("{} Task {} not found", "✗".red(), id),
                Err(e) => eprintln!("{} Error: {}", "✗".red(), e),
            }
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max-1])
    } else {
        s.to_string()
    }
}