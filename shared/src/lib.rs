//! Shared code for core-utils
//! 
//! This module provides shared code (if needed) for all of the core-utils projects.
//! 
//! No shared code exists yet. The add() function is just a placeholder.

/// Placeholder function that just adds two usize values.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
