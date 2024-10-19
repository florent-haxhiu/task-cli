use rusqlite::Error;

use rusqlite::Connection;

use crate::commands::task::Task;

pub fn connect_to_db() -> Result<Connection, Error> {
    let conn = Connection::open("task-cli-db.db")?;
    Ok(conn)
}

pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done BLOB
        )",
        (),
    )?;

    Ok(())
}

pub fn remove_all_inside_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let _ = conn.execute("DELETE FROM tasks", ());

    Ok(())
}

pub fn delete_specific_task(
    conn: &Connection,
    id: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("DELETE FROM tasks WHERE id=(?1)")?;

    stmt.execute([id])?;

    Ok(())
}

pub fn add_task(conn: &Connection, args: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let add = conn.execute(
        "INSERT INTO tasks (id, name, done) VALUES (?1, ?2, ?3)",
        (&args.id, &args.name, &args.done),
    );

    match add {
        Ok(_add) => {
            println!("Task added")
        }
        _ => {
            eprintln!("Error")
        }
    }

    Ok(())
}

pub fn show_all_tasks(conn: &Connection) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, name, done FROM tasks")?;

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    let mut data: Vec<Task> = Vec::new();

    for task in task_iter {
        match task {
            Ok(task) => {
                println!("{task}");
                data.push(task);
            }
            _ => {
                eprintln!("What to show?");
            }
        }
    }

    Ok(data)
}
