// Task CLI
use clap::Parser;
use rusqlite::Result;
use table::table::connect_to_db;

use crate::commands::task::Task;
use crate::table::table::create_table;

mod commands;
mod lib;
mod table;

fn main() -> Result<()> {
    let conn = connect_to_db().unwrap();

    let _ = create_table(&conn);

    let args = Task::parse();
    println!("{:#?}", args);

    conn.execute(
        "INSERT INTO tasks (id, name, done) VALUES (?1, ?2, ?3)",
        (&args.id, &args.name, &args.done),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, done FROM tasks")?;

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
