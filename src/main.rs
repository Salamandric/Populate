use rand::RngExt;
use rusqlite::{Connection, Result, params};

mod doll;
use doll::Doll;

fn main() -> Result<()> {
    println!("Hello, world!");

    let doll_fnames = ["John", "Jane", "Jack", "Jill"];
    let doll_lnames = ["Doe", "Smith", "Johnson", "Brown"];

    let mut rng:rand::prelude::ThreadRng = rand::rng();

    let fnum: usize = rng.random_range(0..doll_fnames.len());
    let lnum: usize = rng.random_range(0..doll_lnames.len());

    println!("{}{}", fnum, lnum);

    let newdoll: Doll = Doll {
        id: 0,
        fname: String::from(doll_fnames[fnum]),
        lname: String::from(doll_lnames[lnum]),
        needs: None
    };

    println!("New doll created: {} {}", newdoll.fname, newdoll.lname);

    let conn =  Connection::open("dolls.db3")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS dolls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fname TEXT NOT NULL,
            lname TEXT NOT NULL,
            needs BLOB
        )",
        (), //no params
    )?;

    conn.execute(
        "INSERT INTO dolls (fname, lname, needs) VALUES (?1, ?2, ?3)",
            (&newdoll.fname, &newdoll.lname, &newdoll.needs),
    )?;

    println!("Doll Added");

    let mut stmt = conn.prepare("SELECT id, fname, lname, needs FROM dolls")?;
    let doll_iter = stmt.query_map([], |row| {
        Ok(Doll {
            id: row.get(0)?,
            fname: row.get(1)?,
            lname: row.get(2)?,
            needs: row.get(3)?,
        })
    })?;

    for doll in doll_iter {
        println!("Found doll: {:?}", doll?);
    }

    Ok(())
}