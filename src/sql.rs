

use welds::{self, Client, connections::sqlite::{self, SqliteClient}, exts::VecStateExt, query::builder::QueryBuilder, state::DbState};
use crate::doll::{Doll, DollNames};

const DATABASEPATH: &'static str = "dolls.db3";

const DOLLSCHEMA: &'static str = 
    "CREATE TABLE IF NOT EXISTS dolls (
    doll_id TEXT PRIMARY KEY UNIQUE,
    fname TEXT NOT NULL,
    lname TEXT NOT NULL,
    sex TEXT NOT NULL,
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

pub async fn add_doll(doll: Doll) {
    let conn = client_new().await;

    let mut newdoll = DbState::new_uncreated(doll);
    newdoll.save(&conn).await.expect("Failed to add doll");
}

pub async fn add_doll_many(list: Vec<Doll>) {
    let conn = client_new().await;

    for doll in list {
        let mut newdoll = DbState::new_uncreated(doll);
        newdoll.save(&conn).await.expect("Failed to add doll from list")
    }
}

pub async fn list_dolls() -> Vec<Doll> {
    
    let conn = client_new().await;
    create_table_if_not_exists(&conn).await;

    let list_query = Doll::all().order_by_asc(|d| d.id);

    let stmt = list_query.run(&conn).await.expect("Couldn't get dolls");
        
    let list = stmt.into_inners();
        
    list
}
 pub async fn random_name_list(query: QueryBuilder<DollNames>) -> Vec<DollNames> {

    let conn = client_new().await;
    create_table_if_not_exists(&conn).await;
    query.run(&conn).await.expect("Couldn't get name list").into_inners()
 }
