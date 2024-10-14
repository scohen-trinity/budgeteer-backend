use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;

async fn get_budgets() -> Json<Vec<Budget>> {
    let test: Vec<Budget> = vec![
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
            icon: String::from("favorite"),
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
    ];
    
    Json(test)
}

pub fn budget_routes() -> Router {
    Router::new().route("/getBudgets", get(get_budgets))
}

#[derive(Serialize)]
struct Budget {
    id: u64,
    name: String,
    participants: Vec<String>,
    balance: u64,
    icon: String,
}
