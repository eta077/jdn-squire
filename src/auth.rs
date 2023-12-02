use async_trait::async_trait;

use axum_login::{AuthUser, AuthnBackend, UserId};

use serde::Deserialize;

use thiserror::Error;

/// The simplest of users.
#[derive(Clone, Debug)]
pub struct SimpleUser {
    pub id: i64,
    pub username: String,
    pub password: String,
}

impl AuthUser for SimpleUser {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

/// Username/password credentials.
#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Any error that can occur during authentication.
#[derive(Debug, Error)]
#[error("{msg}")]
pub struct AuthError {
    msg: String,
}

/// Only allows one hard-coded credential value to be valid.
#[derive(Clone)]
pub struct SimpleBackend {}

#[async_trait]
impl AuthnBackend for SimpleBackend {
    type User = SimpleUser;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if creds.username == "tester" && creds.password == "Squ!r3" {
            Ok(Some(SimpleUser {
                id: 1,
                username: String::from("tester"),
                password: String::from("Squ!r3"),
            }))
        } else {
            Err(AuthError {
                msg: String::from("Invalid username/password"),
            })
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        if *user_id == 1 {
            Ok(Some(SimpleUser {
                id: 1,
                username: String::from("tester"),
                password: String::from("Squ!r3"),
            }))
        } else {
            Err(AuthError {
                msg: String::from("Unknown user"),
            })
        }
    }
}
