use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Budget {
    pub id: u64,
    pub name: String,
    pub participants: Vec<String>,
    pub balance: u64,
    pub icon: String,
}