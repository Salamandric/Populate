use std::time::Instant;

use welds::prelude::*;
use uuid::{Timestamp, Uuid, uuid};


#[derive(Clone, Debug, WeldsModel)]
#[welds(table = "dolls")]
pub struct Doll {
    #[welds(primary_key, rename="doll_id")]
    pub id: i32,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub data: Vec<u8>  //hunger[0] mood[1] energy[2]
}
#[derive(WeldsModel)]
#[welds(readonly, table = "doll_names")]
pub struct DollNames {
    pub name: String,
    pub female: bool,
    pub male: bool,
    pub neutral: bool,
    pub first_name: bool,
    pub last_name: bool
}


impl Doll {
    pub fn to_string(&self) -> String {
        //let format_name = format!("{} {}", self.fname, self.lname);


        let doll_format = format!(
            "Doll {{ID:{}|Name: {:12} {:12}|Gender: {:8}|Hunger: {:9} |Mood: {:9} |Energy: {:9} }}", 
            self.id,
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
                match self.data[0] {
                    ..0x40 => "Stuffed".to_owned(),
                    ..0x80 => "Full".to_owned(),
                    ..0xC0 => "Hungry".to_owned(),
                    ..=0xFF => "Starving".to_owned(),
                }},
            1 => {
                match self.data[1] {
                    ..0x40 => "Blissful".to_owned(),
                    ..0x80 => "Happy".to_owned(),
                    ..0xC0 => "Sad".to_owned(),
                    ..=0xFF => "Depressed".to_owned(),
                }},
            2 => {
                match self.data[2] {
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

pub fn create_new_doll(parents: Option<(Doll, Doll)>) {


    let dollid = Uuid::now_v7().as_hyphenated().to_string();
    
    if let Some(parents) = parents {

    }
}