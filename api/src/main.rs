use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use controllers::budget_controller::budget_routes;

#[tokio::main]
async fn main() {
    // Create the CORS layer to allow requests from any origin
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .nest("/api", budget_routes())
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str { "Hello Axum!" }
