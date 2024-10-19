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

// pub fn show_all_tasks(conn: &Connection) -> () {
//     let mut stmt = conn.prepare("SELECT id, name, done FROM tasks").unwrap();

//     let task_iter = stmt
//         .query_map([], |row| {
//             Ok(Task {
//                 id: row.get(0)?,
//                 name: row.get(1)?,
//                 done: row.get(2)?,
//             })
//         })
//         .unwrap();
//     task_iter
// }

//pub fn get_task_from_db(conn: &Connection, id: &i32) -> Result<()> {
//    let tasks = conn.execute(
//        "SELECT
//            id, name, done
//        FROM tasks
//            WHERE id=(?1)
//        ",
//        &id,
//    )?;
//
//    let task = None;
//
//    match tasks {
//        Ok(tasks) => task = tasks.unwrap(),
//    }
//
//    Ok(())
//}
