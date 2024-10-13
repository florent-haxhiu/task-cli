// Task CLI

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
//#[command(version, about, long_about = None)]
struct Task {
    id: i32,
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = false)]
    done: bool,
}

//#[derive(Debug, Clone)]
//struct Tasks {
//    data: Vec<Task>,
//}

//impl Tasks {
//    fn add_task(&mut self, task: &Task) {
//        let vec = &mut self.data;
//        vec.push(task.clone());
//    }
//}

fn main() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE task (   
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done TEXT
        )",
        (),
    )?;

    let args = Task::parse();

    let is_done = if args.done == false { 'N' } else { 'Y' };

    conn.execute(
        "INSERT INTO task (id, name, done) VALUES (?1, ?2, ?3)",
        (&args.id, &args.name, &is_done.to_string()),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, done FROM task")?;

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    for t in task_iter {
        println!("Found task {:#?}", t.unwrap());
    }
    Ok(())
}
