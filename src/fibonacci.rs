use std::sync::{Arc, Mutex};

use thiserror::Error;

/// A container for the information needed to produce the next Fibonacci number.
#[derive(Clone, Copy, Debug)]
pub struct FibonacciState {
    prev: u128,
    curr: u128,
}

impl Default for FibonacciState {
    fn default() -> Self {
        Self::new()
    }
}

impl FibonacciState {
    /// Creates a new FibonacciState instance.
    pub fn new() -> Self {
        Self { prev: 1, curr: 0 }
    }

    /// Generates the next number in the Fibonacci sequence.
    /// Returns an error if the result is greater than u128::MAX.
    pub fn next(&mut self) -> Result<u128, FibonacciError> {
        let next = self
            .curr
            .checked_add(self.prev)
            .ok_or(FibonacciError::AdditionOverflow)?;
        self.prev = self.curr;
        self.curr = next;
        Ok(next)
    }
}

/// An enumeration of errors that can occur while determining the next Fibonacci number.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum FibonacciError {
    #[error("unable to lock fibonacci state")]
    LockError,
    #[error("addition overflow occurred")]
    AdditionOverflow,
}

/// Calculates the next number in the Fibonacci sequence based on the given state.
pub fn next_fibonacci(
    current_fibonacci: Arc<Mutex<FibonacciState>>,
) -> Result<u128, FibonacciError> {
    current_fibonacci
        .lock()
        .map_err(|_| FibonacciError::LockError)?
        .next()
}

mod tests {
    #[test]
    pub fn test_next_fibonacci() {
        use super::*;

        let state = Arc::new(Mutex::new(FibonacciState::new()));
        let result = next_fibonacci(state);

        assert_eq!(result, Ok(1))
    }

    #[test]
    pub fn test_next_fibonacci_overflow() {
        use super::*;

        let state = Arc::new(Mutex::new(FibonacciState {
            prev: 205697230343233228174223751303346572685,
            curr: 332825110087067562321196029789634457848,
        }));
        let result = next_fibonacci(state);

        assert_eq!(result, Err(FibonacciError::AdditionOverflow))
    }
}
