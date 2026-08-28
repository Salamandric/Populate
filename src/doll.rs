use async_sqlite::Client;

#[derive(Debug, Clone)]
pub struct Doll {
    pub id: u32,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub needs: Vec<u8>, //hunger[0] mood[1] energy[2]
}

impl Doll {
    pub fn to_string(&self) -> String {
        //let format_name = format!("{} {}", self.fname, self.lname);

        let format_id = format!("{:08}", self.id);
        let (id0,id1) = format_id.split_at(4);

        let doll_format = format!(
            "Doll {{ID:{:04}-{:04}|Name: {:12} {:12}|Gender: {:8}|Hunger: {:9} |Mood: {:9} |Energy: {:9} }}", 
            id0, id1,
            self.fname,
            self.lname, 
            self.gender,
            self.get_need_status(0),    //hunger
            self.get_need_status(1),    //mood
            self.get_need_status(2)     //energy
        );

        return doll_format;
    }

    pub fn get_need_status(&self, index: i32) -> String {
        match index {
            0 => {
                match self.needs[0] {
                    ..0x40 => "Stuffed".to_owned(),
                    ..0x80 => "Full".to_owned(),
                    ..0xC0 => "Hungry".to_owned(),
                    ..=0xFF => "Starving".to_owned(),
                }},
            1 => {
                match self.needs[1].to_owned(){
                    ..0x40 => "Blissful".to_owned(),
                    ..0x80 => "Happy".to_owned(),
                    ..0xC0 => "Sad".to_owned(),
                    ..=0xFF => "Depressed".to_owned(),
                }},
            2 => {
                match self.needs[2] .to_owned(){
                    ..0x40 => "Pumped".to_owned(),
                    ..0x80 => "Refreshed".to_owned(),
                    ..0xC0 => "Tired".to_owned(),
                    ..=0xFF => "Exhausted".to_owned(),
            }},
            _ => panic!("Looked for an Invalid Doll Need"),
        }
    }
}

/*
--------------------------
Functions related to Dolls
--------------------------
*/
