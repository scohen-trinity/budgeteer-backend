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
