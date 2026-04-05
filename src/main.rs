use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

//the main CLI struct 
#[derive(Parser)]
#[command(name = "JARVIS", about = "Your personal assistant")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

//subcommands
#[derive(Subcommand)]
enum Commands {
	//add a new task
 	 Add {
		//task description
		name: String,
		// due date
		#[arg(long)]
		due: Option<String>,
	     },
	    ///lists all the tasks
	   List,
}

//the main function 
fn main() -> Result<()> {
	let cli = Cli::parse();

	// Open (or creates) the database in my home directory
	let db_path = dirs_home().unwrap_or_else(|| ".".into()) + "/jarvis.db";
	let conn = Connection::open(&db_path)?;

	//Create the task table if it doesn't exist
 	conn.execute_batch("
		CREATE TABLE IF NOT EXISTS tasks (
			id	INTEGER PRIMARY KEY AUTOINCREMENT,
			name 	TEXT NOT NULL,
			due 	TEXT,
			done 	INTEGER DEFAULT 0,
			created	TEXT DEFAULT (datetime('now'))
			);
			   ")?;

	match cli.command {
		Commands::Add { name, due } => {
			conn.execute(
			    "INSERT INTO tasks (name, due) VALUES (?1, ?2)",
			    (&name, &due),
			            )?;
			println!("Added: {}", name);
		if let Some(d) = due {
			println!("    Due: {}", d);
						}
			  }
		Commands::List => {
			let mut stmt = conn.prepare(
				"SELECT id, name, due, done FROM tasks ORDER BY due ASC NULLS LAST"
						   )?;
			let tasks: Vec<(i64, String, Option<String>, bool)> = stmt
				.query_map([], |row| {
					Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get::<_, i32>(3)? != 0))
						     })?
			 	.filter_map(|r| r.ok())
				.collect();

			if tasks.is_empty() {
			println!("No tasks yet. Add one with: jarvis add \"task name\"");
		         } else {
			   println!("\n------ your tasks ------------------------");
			     for (id, name, due, done) in tasks {
			     let status = if done { "v" } else { "o" };
	                                 		     let due_str = due.unwrap_or_else(|| "no due date".into());
			   println!("  {} [{}]  {} - {}", status, id, name, due_str);
     										   }
			println!("----------------------------------\n");
				}
   			}
		     }
		     
                    Ok(())
}

fn dirs_home() -> Option<String> {
	std::env::var("HOME").ok()
}
