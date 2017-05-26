// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the playing field.

use piston_window::Context;
use piston_window::G2d;
use piston_window::Line;
use piston_window::Transformed;
use piston_window::UpdateArgs;

use color;
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
    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        let radius: f64 = 1.0;
        let line = Line::new(color::GRAY, radius);

        // Draw the center line.
        let position_x: f64 = (self.width as f64) / 2.0 - radius;
        let number_of_dashes: u32 = 10;
        let height: f64 = (self.height as f64) / ((number_of_dashes as f64) * 2.0 - 1.0);
        for i in 0..number_of_dashes {
            let position_y: f64 = (i as f64) * height * 2.0;
            let transformation = context.transform.trans(position_x, position_y);
            line.draw([0.0, 0.0, 0.0, height], &context.draw_state, transformation, graphics);
        }

        // Draw the top line.
        let line = Line::new(color::WHITE, radius);
        let transformation = context.transform.trans(0.0, 0.0 + radius);
        line.draw([0.0, 0.0, self.width as f64, 0.0], &context.draw_state, transformation, graphics);

        // Draw the ball.
        self.ball.draw(&context, graphics);
    }

    /// Update the field state.
    pub fn update(&mut self, update_arguments: &UpdateArgs) {
        self.ball.update(update_arguments.dt, self.width, self.height);
    }
}
