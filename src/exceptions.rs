//! Custom error types for the Spart.
//!
//! This module defines custom errors used throughout Spart for consistent error reporting.
//! The errors in this module are intended to provide clear messages when
//! invalid operations are attempted.

use std::error::Error;
use std::fmt;

/// Represents errors specific to Spart.
#[derive(Debug)]
pub enum SpartError {
    /// Occurs when an invalid dimension is requested.
    InvalidDimension {
        /// The dimension that was requested.
        requested: usize,
        /// The maximum number of dimensions available.
        available: usize,
    },
    /// Occurs when an invalid capacity is provided.
    InvalidCapacity {
        /// The capacity value that was provided.
        capacity: usize,
    },
}

impl fmt::Display for SpartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpartError::InvalidDimension {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Invalid dimension: requested {}, but only {} dimensions available",
                    requested, available
                )
            }
            SpartError::InvalidCapacity { capacity } => {
                write!(
                    f,
                    "Invalid capacity: {}. Capacity must be greater than zero.",
                    capacity
                )
            }
        }
    }
}

impl Error for SpartError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_dimension_display() {
        let err = SpartError::InvalidDimension {
            requested: 3,
            available: 2,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid dimension: requested 3, but only 2 dimensions available"
        );
    }

    #[test]
    fn test_invalid_capacity_display() {
        let err = SpartError::InvalidCapacity { capacity: 0 };
        assert_eq!(
            format!("{}", err),
            "Invalid capacity: 0. Capacity must be greater than zero."
        );
    }
}
