use rusqlite::{Connection, Result};

pub struct Challenge {
    pub name: String,
    pub ctf_name: String,
    pub solved: u8,
}

impl Challenge {
    pub fn new(name: String, ctf_name: String) -> Challenge {
        Challenge {
            name: name,
            ctf_name: ctf_name,
            solved: 0,
        }
    }

    pub fn add_to_db(&self, conn: &Connection) -> Result<()> {
        let mut stmt =
            conn.prepare("INSERT INTO challenges (name, ctf_name, solved) VALUES (?, ?, ?)")?;
        stmt.execute(&[&self.name, &self.ctf_name, &self.solved.to_string()])?;
        Ok(())
    }
}
