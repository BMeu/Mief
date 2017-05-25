// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the playing field.

use piston_window::Context;
use piston_window::G2d;
use piston_window::UpdateArgs;

use elements::Ball;

/// The playing field of the game.
pub struct Field {
    /// The ball used for playing.
    ball: Ball,

    /// The height of the field.
    height: u32,

    /// The width of the field.
    width: u32,
}

impl Field {
    /// Initialize a new playing field with the given size.
    pub fn new(size: [u32; 2]) -> Field {
        Field {
            ball: Ball::new(size),
            height: size[1],
            width: size[0],
        }
    }

    /// Draw the field with its contents.
    pub fn draw(&mut self, context: &Context, graphics: &mut G2d) {
        self.ball.draw(context, graphics);
    }

    /// Resize the field.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.height = height;
        self.width = width;
    }

    /// Update the field state.
    pub fn update(&mut self, update_arguments: &UpdateArgs) {
        self.ball.update(update_arguments.dt, self.width, self.height);
    }
}
