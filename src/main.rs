use std::time::Instant;

use rand::{random_range};

mod doll;
mod sqlhandler;
use doll::Doll;
use crate::sqlhandler::SqlHandler;

fn main() {
    let now = Instant::now();

    let doll_handler = SqlHandler::new();


    //      Test Commands
    //add_and_list(doll_handler);
    count_names(doll_handler);

    let elapsed: f64 = now.elapsed().subsec_millis().try_into().expect("msg");
    println!("Program complete in {} seconds", elapsed/1000.);
}

fn count_names(handler: SqlHandler) {
    println!("Your Masculine First Name is: {}", handler.get_random_name("Masculine, FirstName"));
    println!("Your Feminine First Name is: {}", handler.get_random_name("Feminine, FirstName"));
    println!("Your Neutral First Name is: {}", handler.get_random_name("Neutral, FirstName"));
    println!("Your Surname is: {}", handler.get_random_name("LastName"));
}

fn add_and_list(handler: SqlHandler) {
    
    let doll_fnames = ["John", "Jane", "Jack", "Jill"];
    let doll_lnames = ["Doe", "Smith", "Johnson", "Brown"];
    let doll_genders = ["Male", "Female", "Other"];
    
    let doll_quota = 1;

    for _i in 0..doll_quota {

        let fnum: usize = random_range(0..doll_fnames.len());
        let lnum: usize = random_range(0..doll_lnames.len());
        let gend = {
            match random_range(0..100){
                ..49 => doll_genders[0],
                ..99 => doll_genders[1],
                _ => doll_genders[2]
            }.to_string()
        };

        let val0 = random_range(0..255);
        let val1 = random_range(0..255);
        let val2 = random_range(0..255);

        let newdoll: Doll = Doll {
            id: 0,
            fname: String::from(doll_fnames[fnum]),
            lname: String::from(doll_lnames[lnum]),
            gender: gend,
            needs: vec![val0,val1,val2],
        };

        handler.add_doll(newdoll);

    }

    let doll_list = handler.list_dolls();
    
    println!("List obtained");

    for doll in doll_list {
        println!("{}", doll.to_string());
    }


}