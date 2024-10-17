use serde::Serialize;

#[derive(Serialize)]
pub struct GetBudgetDTO {
    pub name: String,
    pub participants: Vec<String>,
    pub icon: u32,
}
