use axum::{
    routing::{get, post},
    Json,
    Router,
};
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static; // remove eventually when converting to database

use models::budget_model::Budget;
use commands::budget_command::{AddBudgetCommand, QuickAddCommand};

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
            icon: 0xe25b,
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
            icon: 0xf07d4,
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
            icon: 0xe07e,
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
            icon: 0xe06d,
        },
    ]);
}

async fn get_budgets() -> Json<Vec<Budget>> {
    let budgets = GLOBAL_TEST.lock().unwrap().clone();
    Json(budgets)
}

async fn add_budget(Json(payload): Json<AddBudgetCommand>) -> Json<Budget> {
    let mut list: MutexGuard<'_, Vec<Budget>> = GLOBAL_TEST.lock().unwrap();
    let tmp: Budget = Budget::new(payload.name, payload.participants, 0, payload.icon);
    let tmp2: Budget = tmp.clone();
    list.push(tmp);
    Json(tmp2)
}

async fn quick_add_amount(Json(payload): Json<QuickAddCommand>) -> Json<u64> {
    println!("Add {} to {}", payload.amt, payload.id);
    Json(1)
}

pub fn budget_routes() -> Router {
    Router::new()
        .route("/getBudgets", get(get_budgets))
        .route("/addBudget", post(add_budget))
        .route("/quickAddAmount", post(quick_add_amount))
}
