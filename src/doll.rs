#[derive(Debug)]
pub struct Doll {
    pub id: i64,
    pub fname: String,
    pub lname: String,
    pub needs: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct DollNeeds {
    pub hunger: u8,
    pub mood:   u8,
    pub energy: u8
}

fn GetNeedStatus(value: u8, name: &str) -> &str {
    return match name {
        "hunger" => {
            match value {
                0x20 => "Engorged",
                0x40 => "Stuffed",
                0x60 => "Full",
                0x80 => "Satiated",
                0xA0 => "Peckish",
                0xC0 => "Hungry",
                0xE0 => "Starving",
                0xFF => "Ravenous",
                _ =>    "Died of Starvation",
            }},
        "mood" => {
            match value {
                0x20 => "Euphoric",
                0x40 => "Blissful",
                0x60 => "Happy",
                0x80 => "Content",
                0xA0 => "Meh",
                0xC0 => "Sad",
                0xE0 => "Depressed",
                0xFF => "Miserable",
                _ =>    "Died of Low Mood"
            }},
        "energy" => {
            match value {
                0x40 => "Pumped",
                0x60 => "Active",
                0x80 => "Alright",
                0xA0 => "Tired",
                0xE0 => "Drained",
                0xFF => "Exhausted",
                _ =>    "Died of Exhaustion"
        }},
        _ => panic!(""),
    };
}