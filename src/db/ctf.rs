use rusqlite::{Connection, Result};

pub struct Ctf {
    pub name: String,
}

impl Ctf {
    pub fn new(name: &String) -> Ctf {
        Ctf {
            name: name.to_string(),
        }
    }

    pub fn add_to_db(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("INSERT INTO ctfs (name) VALUES (?)")?;
        stmt.execute(&[&self.name])?;
        Ok(())
    }
}
