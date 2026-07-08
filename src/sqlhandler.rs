use rusqlite::{Connection, Error, Result};

use crate::doll::Doll;

pub struct SqlHandler {
    pub conn: Connection
}

impl SqlHandler {

    pub fn create_table_if_not_exists(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Dolls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                fname TEXT NOT NULL,
                lname TEXT NOT NULL,
                needs BLOB NOT NULL
            )",
            (), //no params
        ).expect("");

    }

    pub fn list_dolls(&self) -> Result<Vec<Doll>> {

        self.create_table_if_not_exists();
        
        let mut stmt = self.conn.prepare("SELECT id, fname, lname, needs FROM dolls")?;
        let doll_iter = stmt.query_map([], |row| {
            Ok(Doll {
                id: row.get(0)?,
                fname: row.get(1)?,
                lname: row.get(2)?,
                needs: row.get(3)?
            })
        })?;

        let dolls: Result<Vec<Doll>, Error> = doll_iter.collect();
        return dolls;
    }

    pub fn add_doll(&self, newdoll: Doll) {
        self.create_table_if_not_exists();

        self.conn.execute(
            "INSERT INTO dolls (fname, lname, needs) VALUES (?1, ?2, ?3)",
            (newdoll.fname, newdoll.lname, newdoll.needs),
        ).expect("Error adding doll");

        println!("Doll Added");
    }
}
