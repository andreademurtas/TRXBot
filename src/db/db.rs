use rusqlite::{Connection, Result};

pub fn create_ctfs_table() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "DROP TABLE IF EXISTS ctfs",
        ()
    )?;
    conn.execute(
        "CREATE TABLE ctfs (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            challenge TEXT NOT NULL,
            solved INTEGER NOT NULL
        )",
        ()
    )?;
    Ok(())
}
//TODO: Rethink this
