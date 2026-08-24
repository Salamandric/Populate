use rand::random_range;

use crate::{doll::Doll, sqlhandler::SqlHandler};

#[test]
pub fn list_dolls() {
    


    let doll_handler = SqlHandler::new();

    let doll_list = doll_handler.list_dolls();
    
    println!("List obtained");

    for doll in doll_list {
        println!("{}", doll.to_string());
    }
}
#[test]
pub fn add_random_dolls() {
    
    let doll_handler = SqlHandler::new();

    let doll_genders = ["Male", "Female", "Other"];
    
    let doll_quota = 10;

    for _i in 0..doll_quota {
        let gend = {
            match random_range(0..100){
                ..49 => doll_genders[0],
                ..99 => doll_genders[1],
                _ => doll_genders[2]
            }
        };

        let doll_fname: String;

        match gend {
            "Male" => doll_fname = doll_handler.get_random_name("Masculine, FirstName"),
            "Female" => doll_fname = doll_handler.get_random_name("Feminine, FirstName"),
            "Other" => doll_fname = doll_handler.get_random_name("Neutral, FirstName"),
            _ => panic!("Doll gender too large")
        }
        let doll_lname = doll_handler.get_random_name("LastName");

        let val0 = random_range(0..255);
        let val1 = random_range(0..255);
        let val2 = random_range(0..255);

        let newdoll: Doll = Doll {
            id: 0,
            fname: doll_fname,
            lname: doll_lname,
            gender: gend.to_string(),
            needs: vec![val0,val1,val2],
        };

        doll_handler.add_doll(newdoll);

    }

}
