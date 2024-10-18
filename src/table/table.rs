use rusqlite::Error;

use rusqlite::Connection;

pub fn connect_to_db() -> Result<Connection, Error> {
    let conn = Connection::open("task-cli-db.db")?;
    Ok(conn)
}

pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating table tasks");
    conn.execute(
        "CREATE TABLE tasks (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done BLOB
        )",
        (),
    )?;

    Ok(())
}

pub fn drop_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "DROP TABLE tasks",
        ()
    );

    Ok(())
}
