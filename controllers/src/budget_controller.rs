use axum::{
    routing::{get, post},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static; // remove eventually when converting to database
use std::sync::Mutex;
use commands::budget_command::AddBudgetCommand;

lazy_static! {
static ref GLOBAL_TEST: Mutex<Vec<Budget>> = Mutex::new(vec![
        Budget {
            id: 1,
            name: "Balcony Time".to_string(),
            participants: vec![
                String::from("Sam"),
                String::from("Kyle"),
                String::from("Aiden"),
                String::from("Levi"),
                String::from("Sandra"),
            ],
            balance: 999,
            icon: String::from("favorite"),
        },
        Budget {
            id: 2,
            name: "219 Andrews".to_string(),
            participants: vec![
                String::from("Sam"),
                String::from("Jax"),
                String::from("Nate"),
                String::from("Levi"),
            ],
            balance: 50,
            icon: String::from("wallet"),
        },
        Budget {
            id: 3,
            name: "Crack Cardboard".to_string(),
            participants: vec![
                String::from("Sam"),
                String::from("Kyle"),
                String::from("Nate"),
                String::from("Sandra"),
            ],
            balance: 1000000,
            icon: String::from("all_inclusive"),
        },
        Budget {
            id: 4,
            name: "Boston".to_string(),
            participants: vec![
                String::from("Sam"),
                String::from("Kyle"),
                String::from("Raven"),
                String::from("Sandra"),
            ],
            balance: 0,
            icon: String::from("airplane_ticket"),
        },
    ]);
}

async fn get_budgets() -> Json<Vec<Budget>> { 
    Json(GLOBAL_TEST.lock().unwrap().clone())
}

async fn add_budget(Json(payload): Json<AddBudgetCommand>) -> impl IntoResponse {
    // let mut list = GLOBAL_TEST.lock().unwrap();
    println!("name: {}, icon: {}", payload.name, payload.icon);
    // list.push(payload);
    (StatusCode::OK,)
}

pub fn budget_routes() -> Router {
    Router::new()
        .route("/getBudgets", get(get_budgets))
        .route("/addBudget", post(add_budget))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Budget {
    id: u64,
    name: String,
    participants: Vec<String>,
    balance: u64,
    icon: String,
}
