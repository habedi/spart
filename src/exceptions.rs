//! ## Custom Errors for Spart
//!
//! This module defines custom errors and exceptions that are used internally by Spart.

use std::error::Error;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents errors specific to invalid operations or parameters in Spart.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
                    "Invalid dimension: requested {requested}, but only {available} dimensions available"
                )
            }
            SpartError::InvalidCapacity { capacity } => {
                write!(
                    f,
                    "Invalid capacity: {capacity}. Capacity must be greater than zero."
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
