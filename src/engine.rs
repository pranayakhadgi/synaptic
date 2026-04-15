//imports
use crate::db::{Database, Task, TaskStatus};
use chrono::{NaiveDate, Local, Duration};

pub struct Engine {
    db: Database,
}

//--------------------------COMMANDS-------------------------------
impl Engine {
    pub fn new() -> Result<Self, String> {
        Database::open()
            .map(|db| Self { db })
            .map_err(|e| format!("Error opening database: {}", e))
    }

     // returns the task w/ title, due, tags as a string
    pub fn add_task(
        &self,
        title: &str,
        due: Option<NaiveDate>,
        tags: Vec<String>,
    ) -> Result<u64, String> {
        self.db.add_task(title, due, tags)
            .map_err(|e| e.to_string())
    }


    // a bool function that returns if true if the task is completed
    pub fn complete_task(&self, id: u64) -> Result<bool, String> {
        self.db.complete_task(id).map_err(|e| e.to_string())
    }

    // delete a task by id
    pub fn delete_task(&self, id: u64) -> Result<bool, String> {
        self.db.delete_task(id).map_err(|e| e.to_string())
    }

    // list tasks with optional filter for completed tasks
    pub fn list_tasks(&self, show_done: bool) -> Result<Vec<Task>, String> {
        self.db.list_tasks(show_done).map_err(|e| e.to_string())
    }

    pub fn remind(&self) {
        use colored::Colorize;

        let today = Local::now().date_naive();
        let window = today + chrono::Duration::days(3);

        let tasks = match self.db.list_tasks(false) {
        Ok(t) => t,
        Err(_) => return,
    }; 
    
    //filters to only task with a due date within th ewindow
    let upcoming: Vec<&Task> = tasks.iter().filter(|t|  {
        t.due_date.map(|d| {
            let due = d.date_naive();
            due <= window
        }).unwrap_or(false)
    }).collect();

    if upcoming.is_empty() {
        return; // stays silent
    }

    println!("\n{} {}", "synaptic".bold(), "__ upcoming _________".dimmed());
    for task in upcoming {
        let due_naive = task.due_date.unwrap().date_naive();

        //the diff helps filter out the tasks due in different days
        let diff = (due_naive - today).num_days(); 

        let due_label = if diff < 0 {
            format!("{}", "overdue".red().bold())
        } else if diff == 0 {
            format!("{}", "today".yellow().bold())
        } else if diff == 1 {
            format!("{}", "tomorrow".yellow())
        } else {
            format!("{}", format!("in {} days", diff).blue())
        };

        println!("  {} {}  {}",
        "o".dimmed(),
        task.title.white(),
        due_label
        );
    }
    println!();
    }

    pub fn init_shell_hook(&self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into() );
        let bashrc = format!("{}/.bashrc", home);
        let hook = "\n# synaptic - surface upcoming tasks on terminal open\n
        if command -v synaptic &>/dev/null; then synaptic remind; fi\n";

        match std::fs::read_to_string(&bashrc) {
            Ok(contents) => {
                if contents.contains("synaptic remind") {
                    println!("{} Shell hook already installed.", "v".green());
                    return();
                }
            }
            Err(_) => {}
        }

        match std::fs::OpenOptions::new().append(true).create(true).open(&bashrc) {
            Ok(mut file) => {
                use std::io::Write;
                file.write_all(hook.as_bytes()).expect("Failed to write hook");
                println!("{} Shell hook installed.", "v".green());
                println!(" Restart your terminal or run : {}", "source ~/.bashrc".cyan());
            }
            Err(e) => eprintln!("{} Failed to write to .bashrc: {}", "x".red(), e),
        }
    }
}