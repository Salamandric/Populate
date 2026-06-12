use rand::RngExt;

use doll::Doll;
mod doll;

fn main() {
    println!("Hello, world!");

    let doll_fnames = ["John", "Jane", "Jack", "Jill"];
    let doll_lnames = ["Doe", "Smith", "Johnson", "Brown"];

    let mut rng:rand::prelude::ThreadRng = rand::rng();

    let fnum: usize = rng.random_range(0..doll_fnames.len());
    let lnum: usize = rng.random_range(0..doll_lnames.len());

    println!("{}{}", fnum, lnum);

    let newdoll: Doll = Doll {
        doll_id: 1,
        fname: String::from(doll_fnames[fnum]),
        lname: String::from(doll_lnames[lnum]),
        born: 1959,
        died: None,
    };

    println!("New doll created: {} {}", newdoll.fname, newdoll.lname);
}