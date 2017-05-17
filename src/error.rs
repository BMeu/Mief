// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Error handling.

use std::fmt;

/// A specialized `Result` type for _Mief_.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A wrapper type for all errors caused by _Mief_.
#[derive(Clone, Debug)]
pub enum Error {
    /// Errors caused by Piston.
    Piston(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Piston(ref error) => error.fmt(formatter),
        }
    }
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Piston(_) => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Piston(ref error) => error,
        }
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Piston(error)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as ErrorTrait;
    use super::*;

    #[test]
    fn cause_piston() {
        let message: &str = "Piston Failure";
        let error = Error::Piston(String::from(message));
        assert!(error.cause().is_none(), "Piston errors do not have a cause.");
    }

    #[test]
    fn description_piston() {
        let message: &str = "Piston Failure";
        let error = Error::Piston(String::from(message));
        assert_eq!(error.description(), String::from(message));
    }

    #[test]
    fn fmt_display_piston() {
        let message: &str = "Piston Failure";
        let error = Error::Piston(String::from(message));
        assert_eq!(format!("{}", error), message);
    }

    #[test]
    fn from_string() {
        let message = String::from("Piston Failure");
        match Error::from(message) {
            Error::Piston(_) => assert!(true, "Expected Piston failure.")
        }
    }
}
