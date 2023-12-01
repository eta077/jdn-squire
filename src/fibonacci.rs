use std::sync::{Arc, Mutex};

use thiserror::Error;

/// A container for the information needed to produce the next Fibonacci number.
#[derive(Clone, Copy)]
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
#[derive(Error, Debug)]
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
