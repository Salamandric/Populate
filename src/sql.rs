use rand::random_range;
use welds::{self, Client, WeldsError, connections::sqlite::{self, SqliteClient}, exts::VecStateExt, state::DbState};
use crate::doll::{Doll, DollNames};

const DATABASEPATH: &'static str = "dolls.db3";

const DOLLSCHEMA: &'static str = 
    "CREATE TABLE IF NOT EXISTS dolls (
    doll_id INTEGER PRIMARY KEY,
    fname TEXT NOT NULL,
    lname TEXT NOT NULL,
    gender TEXT NOT NULL,
    data BLOB NOT NULL )";


async fn client_new() -> SqliteClient {
    sqlite::connect(DATABASEPATH).await.expect("Failed to connect to database")
}

async fn create_table_if_not_exists(conn: &impl Client) {
    conn.execute(
        DOLLSCHEMA,
        &[] //no params
    ).await.expect("error creating dolls table");
}

pub async fn add_doll(doll: &mut DbState<Doll>) {
    let conn = client_new().await;
    doll.save(&conn).await.expect("Failed to add doll");        
}

pub async fn list_dolls() -> Vec<Doll> {
    
    let conn = client_new().await;
    create_table_if_not_exists(&conn).await;

    let list_query = Doll::all().order_by_asc(|d| d.id);

    let stmt = list_query.run(&conn).await.expect("Couldn't get dolls");
        
    let list = stmt.into_inners();
        
    list
}
 pub async fn get_random_name(conn: &impl Client, name: DollNames) -> String {

        let query = DollNames::
        where_col(|n|n.female.equal(name.female))
        .where_col(|n|n.male.equal(name.male))
        .where_col(|n|n.neutral.equal(name.neutral))
        .where_col(|n|n.first_name.equal(name.first_name))
        .where_col(|n|n.last_name.equal(name.last_name))
        .run(conn).await.expect("Couldn't get doll names");

        let num = rand::random_range(0..query.iter().len());

        query[num].name.to_owned()
    }

pub struct SqlHandler {
    pub conn: welds::connections::sqlite::SqliteClient
}

impl SqlHandler {
    
    pub async fn new() -> SqlHandler {
        Self {
            conn: sqlite::connect("dolls.db3").await.expect("Failed to connect to database")
        }
    }


    

    
}
