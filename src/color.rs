// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Color definitions.

/// `#000000`, `100%` opacity.
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// `#808080`, `100%` opacity.
pub const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

/// '#00ff00', '100%' capacity.
#[cfg(feature = "display-fps")]
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

/// `#ffffff`, `100%` opacity.
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
