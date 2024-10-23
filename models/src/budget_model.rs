use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Budget {
    pub id: u64,
    pub name: String,
    pub participants: Vec<String>,
    pub balance: f64,
    pub icon: u32,
}

impl Budget {
    pub fn new(name: String, participants: Vec<String>, balance: f64, icon: u32) -> Budget {
        Budget {
            id: 1,
            name,
            participants,
            balance,
            icon,
        }
    }
}