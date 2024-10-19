use rusqlite::{Connection, Error};

#[derive(Debug, Clone, Copy)]
struct TaskId {
    id: i32,
}

// Get tasks and see which id is free
pub fn get_free_id(conn: &Connection) -> Result<i32, Error> {
    let mut stmt = conn.prepare("SELECT MAX(id) AS max_id FROM tasks")?;
    let mut free_id: i32 = 0;

    let id_iter = stmt.query_map([], |row| Ok(TaskId { id: row.get(0)? }))?;

    for id in id_iter {
        match id {
            Ok(id) => {
                if free_id <= id.id {
                    free_id = id.id + 1
                }
            }
            _ => {
                eprintln!("No ID found")
            }
        }
    }

    Ok(free_id)
}
