use rand::RngExt;
use rusqlite::Connection;

mod doll;
mod sqlhandler;
use doll::Doll;
use crate::sqlhandler::SqlHandler;

fn main() {
    println!("Hello, world!");

    test_program();

}

fn test_program() {
    let doll_fnames = ["John", "Jane", "Jack", "Jill"];
    let doll_lnames = ["Doe", "Smith", "Johnson", "Brown"];

    let mut rng:rand::prelude::ThreadRng = rand::rng();

    let handler =  SqlHandler {conn: Connection::open("dolls.db3").expect("Error opening connection")};
    
    let doll_quota = 10;

    for _i in 0..doll_quota {

        let fnum: usize = rng.random_range(0..doll_fnames.len());
        let lnum: usize = rng.random_range(0..doll_lnames.len());

        let val0 = rng.random_range(0..255);
        let val1 = rng.random_range(0..255);
        let val2 = rng.random_range(0..255);

        let newdoll: Doll = Doll {
            id: 0,
            fname: String::from(doll_fnames[fnum]),
            lname: String::from(doll_lnames[lnum]),
            needs: vec![val0,val1,val2],
        };

        println!("{}", newdoll.to_string());

        handler.add_doll(newdoll);
    }

    let doll_list = handler.list_dolls();
    
    println!("List obtained");

    if let rusqlite::Result::Ok(doll_list) = doll_list {
        for doll in doll_list {
            println!("{}", doll.to_string());
        }
    }
}