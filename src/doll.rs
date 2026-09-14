use std::collections::HashMap;

use rand::{RngExt, random_range, rngs::SmallRng};
use welds::prelude::*;
use uuid::Uuid;

use crate::sql;

#[derive(Clone, Debug, WeldsModel)]
#[welds(table = "dolls")]
pub struct Doll {
    #[welds(primary_key, rename="doll_id")]
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub sex: String,
    pub data: Vec<u8>  //hunger[0] mood[1] energy[2]
}
#[derive(Default, Clone, WeldsModel)]
#[welds(readonly, table = "doll_names")]
pub struct DollNames {
    pub name: String,
    pub female: Option<bool>,
    pub male: Option<bool>,
    pub neutral: Option<bool>,
    pub first_name: Option<bool>,
    pub last_name: Option<bool>
}

#[derive(Debug, Clone)]
pub enum DollSex {
    Male,
    Female,
    Intersex
}

impl Doll {
    pub fn to_string(&self) -> String {
        //let format_name = format!("{} {}", self.fname, self.lname);


        let doll_format = format!(
            "Doll {{ID:{}|Name: {:12} {:12}|Gender: {:8}|Hunger: {:9} |Mood: {:9} |Energy: {:9} }}", 
            self.id,
            self.fname,
            self.lname, 
            self.sex,
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

pub async fn create_dolls_random(quota: i32) -> Vec<Doll> {
    let mut list = vec![];
    for _ in 0..quota {
        list.push(create_doll(None).await)
    }
    list
}

pub async fn create_doll(parents: Option<(Doll, Doll)>) -> Doll {

    let mut rng: SmallRng = rand::make_rng();
    let doll_id = Uuid::now_v7().hyphenated().encode_upper(&mut Uuid::encode_buffer()).to_string();
    

    //add (very unlikely) check if uuid is already taken.


    //jesus christ this is all for a damn sex
    let mut sex_weight: HashMap<&str, f32> = HashMap::new();
    sex_weight.insert("Male", 50.0);
    sex_weight.insert("Female", 50.0);
    sex_weight.insert("Intersex", 1.0);

    let totalweight = {
        let mut weight: f32 = 0.0;
        for val in sex_weight.values() {
            weight = weight + f32::from(*val);
        }
        weight
    };
    let randnum: f32 = rng.random();

    let mut index = randnum * totalweight;

    let chosensex: &str = {
        let mut sex: &str = "";
        for kvp in sex_weight {
            if index <= kvp.1 {
                sex = kvp.0;
                break;
            }
            index -= kvp.1;
        }
        sex
    };
    let doll_sex = {
        match chosensex {
            "Male" => DollSex::Male,
            "Female" => DollSex::Female,
            "Intersex" => DollSex::Intersex,
            _ => panic!("chosensex is not real")
        }
    };
    
    let doll_data: Vec<u8> = vec![
        rng.random(),
        rng.random(),
        rng.random()];

    let doll_fname: String;
    let doll_lname: String;

    // Name generator
    if let Some(parents) = parents {
        todo!("Add case for if doll has parents")
    }
    else {
        let query = {
            match doll_sex {
                DollSex::Male       => DollNames::where_col(|n|n.male.equal(true)),
                DollSex::Female     => DollNames::where_col(|n|n.female.equal(true)),
                DollSex::Intersex   => DollNames::where_col(|n|n.neutral.equal(true))
            }
            .where_col(|n|n.first_name.equal(true))
        };
        let fname_list = sql::random_name_list(query).await;
        let lname_list = sql::random_name_list(DollNames::where_col(|n|n.last_name.equal(true))).await;
        let findex = random_range(0..fname_list.len());
        let lindex = random_range(0..lname_list.len());

        doll_fname = fname_list[findex].clone().name;
        doll_lname = lname_list[lindex].clone().name;
    }  

    let newdoll = Doll {
        id: doll_id,
        fname: doll_fname,
        lname: doll_lname,
        sex: chosensex.to_string(),
        data: doll_data,
    };

    newdoll
    
}
