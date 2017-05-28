// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Quit the application with standardized exit codes.

use std::error::Error as ErrorTrait;
use std::process;

use execution_flow::Error;

/// The exit codes returned by the _Mief_.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Code {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Failure of the Piston game engine (Code: `1`).
    PistonFailure = 1,

    /// Failure during I/O operations (Code: `2`).
    IOFailure = 2,
}

impl From<Code> for i32 {
    fn from(code: Code) -> i32 {
        code as i32
    }
}

/// Quit the program execution. The exit code and message are chosen based on `error`.
pub fn fail_from_error(error: Error) -> ! {
    match error {
        Error::IO(error) => fail_with_message(Code::IOFailure, error.description()),
        Error::Piston(message) => fail_with_message(Code::PistonFailure, &message)
    }
}

/// Quit the program execution with the given `exit_code` and an error `message` explaining the exit.
pub fn fail_with_message(exit_code: Code, message: &str) -> ! {
    println!("Error: {description}", description = message);
    quit(exit_code)
}

/// Quit the program execution with a `Success` exit code.
pub fn succeed() -> ! {
    quit(Code::Success)
}

/// Quit the program execution with the given code.
fn quit<I: Into<i32>>(code: I) -> ! {
    process::exit(code.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code_success() {
        assert_eq!(0, Code::Success.into());
    }

    #[test]
    fn exit_code_piston_failure() {
        assert_eq!(1, Code::PistonFailure.into());
    }

    #[test]
    fn exit_code_io_failure() {
        assert_eq!(2, Code::IOFailure.into());
    }
}
