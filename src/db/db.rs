use rusqlite::{Connection, Result};

pub fn setup_db() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute("DROP TABLE IF EXISTS ctfs", ())?;
    conn.execute("DROP TABLE IF EXISTS challenges", ())?;
    conn.execute(
        "CREATE TABLE ctfs (
            name TEXT PRIMARY KEY,
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE challenges (
            name TEXT PRIMARY KEY,
            ctf_name TEXT NOT NULL,
            solved INTEGER NOT NULL,
            FOREIGN KEY(ctf_name) REFERENCES ctfs(name)
        )",
        (),
    )?;
    Ok(())
}
