use chrono::{DateTime, Utc, NaiveDate};
use rusqlite::{Connection, Result, params};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
	pub id: u64,
	pub title: String,
	pub status: TaskStatus,
	pub due_date: Option<DateTime<Utc>>,
	pub tags: Vec<String>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
	Todo, 
	Done,
}

impl TaskStatus {
	pub fn as_str(&self) -> &'static str {
		match self {
			TaskStatus::Todo => "todo",
			TaskStatus::Done => "done",
	        	}
		}

	pub fn from_str(s: &str) -> Option<Self> {
		match s {
		     "todo" => Some(TaskStatus::Todo),
		     "done" => Some(TaskStatus::Done),
		      _ => None,
		}
	 }
}

pub struct Database {
	conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let mut db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&mut self) -> Result<()> {
        // Base table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'todo',
                due_date TEXT,
                tags TEXT DEFAULT '[]',
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;
        Ok(())
    }
    
    pub fn add_task(&self, title: &str, due: Option<NaiveDate>, tags: Vec<String>) -> Result<u64> {
        let due_str = due.map(|d| d.format("%Y-%m-%d").to_string());
        let tags_json = serde_json::to_string(&tags).unwrap_or_else(|_| "[]".to_string());

        self.conn.execute(
            "INSERT INTO tasks (title, due_date, tags) VALUES (?1, ?2, ?3)",
            params![title, due_str, tags_json],
        )?;

        Ok(self.conn.last_insert_rowid() as u64)
    }
    
    pub fn complete_task(&self, id: u64) -> Result<bool> {
        let changed = self.conn.execute(
            "UPDATE tasks SET status = 'done' WHERE id = ?1", [id],
        )?;
        Ok(changed > 0)
    }

    pub fn delete_task(&self, id: u64) -> Result<bool> {
        let changed = self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1", [id],
        )?;
        Ok(changed > 0)
    }

    pub fn list_tasks(&self, show_done: bool) -> Result<Vec<Task>> {
        let sql = if show_done {
            "SELECT id, title, status, due_date, tags, created_at
            FROM tasks
            ORDER BY
                CASE WHEN status = 'done' THEN 1 ELSE 0 END, 
                created_at DESC"
        } else {
            "SELECT id, title, status, due_date, tags, created_at
            FROM tasks 
            WHERE status = 'todo'
            ORDER BY created_at DESC"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let task_iter = stmt.query_map([], |row| {
            let status_str: String = row.get(2)?;
            let due_str: Option<String> = row.get(3)?;
            let tags_json: String = row.get(4)?;
            let created_at_str: String = row.get(5)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: TaskStatus::from_str(&status_str).unwrap_or(TaskStatus::Todo),
                due_date: due_str.and_then(|d| {
                    NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                        .ok()
                        .map(|nd| nd.and_hms_opt(0,0,0).unwrap().and_utc())
                }),
                tags: serde_json::from_str(&tags_json).unwrap_or_default(),
                created_at,
            })
        })?;

        task_iter.collect()
    }
}
