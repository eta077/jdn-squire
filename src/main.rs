use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex, RwLock};

use axum::error_handling::HandleErrorLayer;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{BoxError, Json, Router};

use axum_login::tower_sessions::{MemoryStore, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder, AuthSession};

use tokio::net::TcpListener;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use tracing::*;

mod auth;
use auth::*;

mod fibonacci;
use fibonacci::FibonacciState;

mod users;
use users::{User, UserError, UserState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let current_fibonacci = Arc::new(Mutex::new(FibonacciState::new()));
    let users = Arc::new(RwLock::new(HashMap::new()));

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(true);

    let auth_backend = SimpleBackend {};
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(auth_backend, session_layer).build());

    let router = Router::new()
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
        )
        .route_layer(login_required!(SimpleBackend, login_url = "/login"))
        .route("/login", post(login))
        .route("/hello", get(hello_world))
        .route(
            "/next",
            post({
                let shared_state = Arc::clone(&current_fibonacci);
                move || next_fibonacci(shared_state)
            }),
        )
        .layer(TraceLayer::new_for_http())
        .layer(auth_service);

    let listener = TcpListener::bind("0.0.0.0:1042").await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

/// Authenticates the user with the given credentials.
pub async fn login(
    mut auth_session: AuthSession<SimpleBackend>,
    Json(creds): Json<Credentials>,
) -> (StatusCode, String) {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        _ => return (StatusCode::FORBIDDEN, String::from("invalid credentials")),
    };

    if auth_session.login(&user).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("login failed"),
        );
    }

    (StatusCode::OK, String::new())
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
