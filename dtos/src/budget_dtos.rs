use serde::Serialize;

#[derive(Serialize)]
pub struct BudgetDTO {
    pub id: u64,
    pub name: String,
    pub participants: Vec<String>,
    pub balance: u64,
    pub icon: u32,
}

#[derive(Serialize)]
pub struct GetBudgetDTO {
    pub name: String,
    pub participants: Vec<String>,
    pub icon: u32,
}
