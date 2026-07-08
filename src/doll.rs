#[derive(Debug)]
pub struct Doll {
    pub id: u32,
    pub fname: String,
    pub lname: String,
    pub needs: Vec<u8>,
}

impl Doll {
    
    pub fn to_string(&self) -> String {
        let format_name = format!("{} {}", self.fname, self.lname);

        let format_id = format!("{:08}", self.id);
        let (id0,id1) = format_id.split_at(4);

        let doll_format = format!(
            "Doll {{ID:{:04}-{:04}|Name: {:15}|Hunger: {:9} |Mood: {:9} |Energy: {:9} }}", 
            id0, id1,
            format_name, 
            self.get_need_status("hunger"), 
            self.get_need_status("mood"), 
            self.get_need_status("energy")
        );
        return doll_format;
    }

    pub fn get_need_status(&self, name: &str) -> String {
        return match name {
            "hunger" => {
                match self.needs[0] {
                    ..0x40 => "Stuffed".to_owned(),
                    ..0x80 => "Full".to_owned(),
                    ..0xC0 => "Hungry".to_owned(),
                    ..=0xFF => "Starving".to_owned(),
                }},
            "mood" => {
                match self.needs[1].to_owned(){
                    ..0x40 => "Blissful".to_owned(),
                    ..0x80 => "Happy".to_owned(),
                    ..0xC0 => "Sad".to_owned(),
                    ..=0xFF => "Depressed".to_owned(),
                }},
            "energy" => {
                match self.needs[2] .to_owned(){
                    ..0x40 => "Pumped".to_owned(),
                    ..0x80 => "Refreshed".to_owned(),
                    ..0xC0 => "Tired".to_owned(),
                    ..=0xFF => "Exhausted".to_owned(),
            }},
            _ => panic!(""),
        };
    }
}