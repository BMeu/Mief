// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Quit the game with standardized exit codes.

use std::process;

use error::Error;

/// The exit codes returned by the _Mief_.
#[derive(Clone, Copy, Debug)]
pub enum ExitCode {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Failure of the Piston game engine (Code: `1`).
    PistonFailure = 1,
}

/// Quit the program execution. The exit code and message are chosen based on `error`.
pub fn fail_from_error(error: Error) -> ! {
    match error {
        Error::Piston(message) => {
            fail_with_message(ExitCode::PistonFailure, &message);
        }
    }
}

/// Quit the program execution with the given `exit_code` and an error `message` explaining the exit.
pub fn fail_with_message(exit_code: ExitCode, message: &str) -> ! {
    println!("Error: {description}", description = message);
    process::exit(exit_code as i32)
}

/// Quit the program execution with a `Success` exit code.
pub fn succeed() -> ! {
    process::exit(ExitCode::Success as i32)
}
