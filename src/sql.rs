use rand::random_range;
use async_sqlite::rusqlite;
use crate::{doll::Doll};


async fn create_table_if_not_exists() {

}


pub async fn list_dolls() -> Vec<Doll> {
    create_table_if_not_exists().await;
    println!("Fetching Dolls");
    [].to_vec()
}
 
pub struct SqlHandler {
    pub conn: rusqlite::Connection
}

impl SqlHandler {

    pub fn new() -> Self {
        Self {conn: rusqlite::Connection::open("dolls.db3").expect("Error opening connection")}
    }
    
    pub fn create_table_if_not_exists(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Dolls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                fname TEXT NOT NULL,
                lname TEXT NOT NULL,
                gender TEXT NOT NULL,
                needs BLOB NOT NULL
            )",
            (), //no params
        ).expect("error creating dolls table");

    }

    pub fn list_dolls(&self) -> Vec<Doll> {
        
        self.create_table_if_not_exists();
        
        let mut stmt = self.conn.prepare("SELECT id, fname, lname, gender, needs FROM dolls").expect("error getting dolls");
        let doll_iter = stmt.query_map([], |row| {
            Ok(Doll {
                id: row.get(0)?,
                fname: row.get(1)?,
                lname: row.get(2)?,
                gender: row.get(3)?,
                needs: row.get(4)?
            })
        }).expect("error mapping dolls");

        let dolls: Result<Vec<Doll>, rusqlite::Error> = doll_iter.collect();
        if let Ok(dolls) = dolls {
            dolls
        }
        else {
            panic!("SqlHandler.list_dolls: result not valid ");
        }
    }

    pub async fn list_dolls_async(&self) -> Vec<Doll> {

        self.create_table_if_not_exists();
        
        let mut stmt = self.conn.prepare("SELECT id, fname, lname, gender, needs FROM dolls").expect("error getting dolls");
        let doll_iter = stmt.query_map([], |row| {
            Ok(Doll {
                id: row.get(0)?,
                fname: row.get(1)?,
                lname: row.get(2)?,
                gender: row.get(3)?,
                needs: row.get(4)?
            })
        }).expect("error mapping dolls");

        let dolls: Result<Vec<Doll>, rusqlite::Error> = doll_iter.collect();
        if let Ok(dolls) = dolls {
            dolls
        }
        else {
            panic!("SqlHandler.list_dolls: result not valid ");
        }
    }

    pub fn add_doll(&self, newdoll: Doll) {

        self.create_table_if_not_exists();

        self.conn.execute(
            "INSERT INTO dolls (fname, lname, gender, needs) VALUES (?1, ?2, ?3, ?4)",
            (newdoll.fname, newdoll.lname, newdoll.gender, newdoll.needs),
        ).expect("Error adding doll");
    }

    pub fn get_random_name(&self, types:&str) -> String {

        self.create_table_if_not_exists();

        let mut query = String::from("Select Name FROM DollNames");

        if types != "" {

            let requirements: Vec<&str> = types.split(", ").collect();

            query += " WHERE ";

            let mut didadd = false;
            
            for ask in requirements {

                match ask {
                    "Masculine" => {
                        if didadd {query += " AND "}
                        query += "Masculine = 1";
                        didadd = true;
                    },
                    "Feminine" => {
                        if didadd {query += " AND "}
                        query += "Feminine = 1";
                        didadd = true;
                    },
                    "Neutral" => {
                        if didadd {query += " AND "}
                        query += "Neutral = 1";
                        didadd = true;
                    },
                    "FirstName" => {
                        if didadd {query += " AND "}
                        query += "FirstName = 1";
                        didadd = true;
                    },
                    "LastName" => {
                        if didadd {query +=" AND "}
                        query += "LastName = 1";
                        didadd = true;
                    },
                    _ => println!("ERROR, \"{}\" Is not a valid name type.", ask)
                }
            }
        }

        println!("Name Query: {}", query);

        let mut stmt = self.conn.prepare(&query).expect("Error with query");
        let name_iter = stmt.query_map([], |row| row.get(0)).expect("Couldn't get rows");

        let name_list: Result<Vec<String>, rusqlite::Error> = name_iter.collect();
        
        if let Ok(name_list) = name_list {
            let nameindex = random_range(0..name_list.len());
            
            name_list[nameindex].clone()
        }
        else {
            panic!("SqlHandler.get_random_name: result not valid");
        }
        

    }
}
