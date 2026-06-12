
mod doll;

fn main() {
    println!("Hello, world!");

    let newdoll = doll::Doll {
        doll_id: 1,
        fname: String::from("Barbie"),
        lname: String::from("Smith"),
        born: 1959,
        died: None,
    };
    println!("New doll created: {} {}", newdoll.fname, newdoll.lname);
}