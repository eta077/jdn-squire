use std::error::Error;
use std::sync::{Arc, Mutex};

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;

use tokio::net::TcpListener;

use tracing::Level;

mod fibonacci;
use fibonacci::FibonacciState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .init();

    let current_fibonacci = Arc::new(Mutex::new(FibonacciState::new()));

    let router = Router::new().route("/hello", get(hello_world)).route(
        "/next",
        post({
            let shared_state = Arc::clone(&current_fibonacci);
            move || next_fibonacci(shared_state)
        }),
    );

    let listener = TcpListener::bind("0.0.0.0:1042").await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

/// The string used to greet the user.
const HELLO_WORLD: &str = "Hello World!";

/// Greets the user with the classic salutation.
async fn hello_world() -> &'static str {
    HELLO_WORLD
}

/// Calculates the next number in the Fibonacci sequence.
pub async fn next_fibonacci(current_fibonacci: Arc<Mutex<FibonacciState>>) -> (StatusCode, String) {
    match fibonacci::next_fibonacci(current_fibonacci) {
        Ok(result) => (StatusCode::OK, result.to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
