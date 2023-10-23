use dotenv::dotenv;
pub mod models;
use axum::{
    routing::{get, post},
    Router,
};
pub mod handlers;
use handlers::{auth, google_authorization};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = dotenv::from_filename(".env");
    // build our application with a single route
    let app = Router::new()
        .route("/", get(google_authorization))
        .route("/auth", post(auth));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
