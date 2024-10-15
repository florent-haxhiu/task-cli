// Task CLI
use rusqlite::{Connection, Result};

use clap::Parser;

use crate::table::table::create_table;

mod table;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Task {
    #[arg(short, long, default_value_t = 1)]
    id: i32,
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = false)]
    done: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let _ = create_table(&conn);

    let args = Task::parse();
    println!("{:#?}", args);

    conn.execute(
        "INSERT INTO task (id, name, done) VALUES (?1, ?2, ?3)",
        (&args.id, &args.name, &args.done),
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
        println!("Found {:#?}", t.unwrap());
    }
    Ok(())
}
