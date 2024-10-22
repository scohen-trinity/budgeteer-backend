use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddBudgetCommand {
    pub name: String,
    pub participants: Vec<String>,
    pub icon: u32,
}

#[derive(Deserialize)]
pub struct UpdateBudgetCommand {

}

#[derive(Deserialize)]
pub struct QuickAddCommand {
    pub id: u64,
    pub amt: f64,
}
