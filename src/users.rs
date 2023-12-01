use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type UserState = Arc<RwLock<HashMap<String, User>>>;

/// A user of the system.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: String,
    name: String,
    age: u8,
}

/// An enumeration of errors that can occur when interacting with user data.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserError {
    #[error("unable to lock user state")]
    LockError,
    #[error("failed to serialize user list")]
    SerializationError,
    #[error("user does not exist for the given ID")]
    UnknownUser,
}

/// Generates a JSON representation of all users.
pub fn get_users(users: UserState) -> Result<String, UserError> {
    let lock_guard = users.read().map_err(|_| UserError::LockError)?;
    let list = lock_guard.values().collect::<Vec<_>>();
    serde_json::to_string_pretty(&list).map_err(|_| UserError::SerializationError)
}

/// Generates a JSON representation for the user with the given ID.
pub fn get_user(users: UserState, id: String) -> Result<String, UserError> {
    let lock_guard = users.read().map_err(|_| UserError::LockError)?;
    let user = lock_guard.get(&id).ok_or(UserError::UnknownUser)?;
    serde_json::to_string_pretty(user).map_err(|_| UserError::SerializationError)
}

/// Creates or updates the user with the given information.
pub fn update_user(users: UserState, user: User) -> Result<(), UserError> {
    let mut lock_guard = users.write().map_err(|_| UserError::LockError)?;
    lock_guard.insert(user.id.to_owned(), user);
    Ok(())
}

mod tests {
    #[test]
    pub fn test_get_users() {
        use super::*;

        let list = HashMap::from([(
            String::from("test"),
            User {
                id: String::from("test"),
                name: String::from("John"),
                age: 31,
            },
        )]);
        let users = Arc::new(RwLock::new(list));
        let result = get_users(users);

        assert_eq!(
            result,
            Ok(String::from(
                r#"[
  {
    "id": "test",
    "name": "John",
    "age": 31
  }
]"#
            ))
        )
    }

    #[test]
    pub fn test_get_user_ok() {
        use super::*;

        let list = HashMap::from([(
            String::from("test"),
            User {
                id: String::from("test"),
                name: String::from("John"),
                age: 31,
            },
        )]);
        let users = Arc::new(RwLock::new(list));
        let result = get_user(users, String::from("test"));

        assert_eq!(
            result,
            Ok(String::from(
                r#"{
  "id": "test",
  "name": "John",
  "age": 31
}"#
            ))
        )
    }

    #[test]
    pub fn test_get_user_unknown() {
        use super::*;

        let list = HashMap::from([(
            String::from("test"),
            User {
                id: String::from("test"),
                name: String::from("John"),
                age: 31,
            },
        )]);
        let users = Arc::new(RwLock::new(list));
        let result = get_user(users, String::from("john"));

        assert_eq!(result, Err(UserError::UnknownUser))
    }

    #[test]
    pub fn test_update_user_new() {
        use super::*;

        let list = HashMap::new();
        let users = Arc::new(RwLock::new(list));
        let user = User {
            id: String::from("test"),
            name: String::from("John"),
            age: 31,
        };
        update_user(Arc::clone(&users), user).unwrap();

        let result = get_users(users);

        assert_eq!(
            result,
            Ok(String::from(
                r#"[
  {
    "id": "test",
    "name": "John",
    "age": 31
  }
]"#
            ))
        )
    }

    #[test]
    pub fn test_update_user_existing() {
        use super::*;

        let list = HashMap::from([(
            String::from("test"),
            User {
                id: String::from("test"),
                name: String::from("John"),
                age: 31,
            },
        )]);
        let users = Arc::new(RwLock::new(list));
        let user = User {
            id: String::from("test"),
            name: String::from("John"),
            age: 21,
        };
        update_user(Arc::clone(&users), user).unwrap();

        let result = get_users(users);

        assert_eq!(
            result,
            Ok(String::from(
                r#"[
  {
    "id": "test",
    "name": "John",
    "age": 21
  }
]"#
            ))
        )
    }
}
