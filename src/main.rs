use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex, RwLock};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

use tokio::net::TcpListener;

use tracing::Level;

mod fibonacci;
use fibonacci::FibonacciState;

mod users;
use users::{User, UserError, UserState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .init();

    let current_fibonacci = Arc::new(Mutex::new(FibonacciState::new()));
    let users = Arc::new(RwLock::new(HashMap::new()));

    let router = Router::new()
        .route("/hello", get(hello_world))
        .route(
            "/next",
            post({
                let shared_state = Arc::clone(&current_fibonacci);
                move || next_fibonacci(shared_state)
            }),
        )
        .route(
            "/users",
            get({
                let shared_state = Arc::clone(&users);
                move || get_users(shared_state)
            })
            .post({
                let shared_state = Arc::clone(&users);
                move |user| update_user(shared_state, user)
            }),
        )
        .route(
            "/user/:id",
            get({
                let shared_state = Arc::clone(&users);
                move |id| get_user(shared_state, id)
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

/// Generates a JSON representation of all users.
pub async fn get_users(users: UserState) -> (StatusCode, String) {
    match users::get_users(users) {
        Ok(result) => (StatusCode::OK, result),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

/// Generates a JSON representation for the user with the given ID.
pub async fn get_user(users: UserState, Path(id): Path<String>) -> (StatusCode, String) {
    match users::get_user(users, id) {
        Ok(result) => (StatusCode::OK, result),
        Err(UserError::UnknownUser) => (StatusCode::NOT_FOUND, UserError::UnknownUser.to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

/// Creates or updates the user with the given information.
pub async fn update_user(users: UserState, Json(user): Json<User>) -> (StatusCode, String) {
    match users::update_user(users, user) {
        Ok(()) => (StatusCode::OK, String::new()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
