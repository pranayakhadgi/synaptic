//imports
use crate::db::{Database, Task, TaskStatus};
use chrono::NaiveDate;

pub struct Engine {
    db: Database,
}

#[derive(Debug, Clone)]
pub struct TaskSummary {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
    pub due_days: Option<i64>,
    pub tags: Vec<String>,
    pub is_overdue: bool,
}
impl Engine {
    pub fn new() -> Result<Self, String> {
        Database::open()
            .map(|db| Self { db })
            .map_err(|e| format!("Error opening database: {}", e))
    }

    //--------------------------COMMANDS-------------------------------

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

    // * FUTURE: Placeholder for task ranking logic once prioritization rules are finalized.
    pub fn rank_tasks_placeholder(&self) -> Result<Vec<Task>, String> {
        self.db.list_tasks(true).map_err(|e| e.to_string())
    }
}
=======
//imports
use crate::db::{Database, Task, TaskStatus};
use chrono::NaiveDate;

pub struct Engine {
    db: Database,
}

#[derive(Debug, Clone)]
pub struct TaskSummary {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
    pub due_days: Option<i64>,
    pub tags: Vec<String>,
    pub is_overdue: bool,
}
impl Engine {
    pub fn new(db_path: &str) -> Result<Self, String> {
        Database::open(db_path)
            .map(|db| Self { db })
            .map_err(|e| format!("Error opening database: {}", e))
    }

    //--------------------------COMMANDS-------------------------------

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
}
>>>>>>> 03b4e01 (added base var for file path)
