use rusqlite::Connection;

/*
*
*
*
*
*/
pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE task (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done BLOB
        )",
        (),
    )?;

    Ok(())
}
