// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the ball.

use piston_window::Context;
use piston_window::Ellipse;
use piston_window::G2d;
use piston_window::Transformed;

use color;

/// The ball used for playing.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ball {
    /// The diameter of the ball.
    diameter: f64,

    /// The current position of the ball: `(x, y)`.
    position: (f64, f64),

    /// The current speed of the ball: `(x, y)`.
    speed: (f64, f64),
}

impl Ball {
    /// Create a new ball.
    pub fn new() -> Ball {
        Ball {
            diameter: 10.0,
            position: (0.0, 0.0),
            speed: (100.0, 100.0),
        }
    }

    /// Draw the ball.
    pub fn draw(&mut self, context: &Context, graphics: &mut G2d) {
        let ball = Ellipse::new(color::WHITE).resolution(100);
        let transformation = context.transform.trans(self.position.0, self.position.1);
        ball.draw([0.0, 0.0, self.diameter, self.diameter], &context.draw_state, transformation, graphics);
    }

    /// Update the ball's position. `dt` is the change in time since the last update, `width` and `height` are the
    /// window's size.
    pub fn update(&mut self, dt: f64, width: u32, height: u32) {
        let progress_x = self.speed.0 * dt;
        let progress_y = self.speed.1 * dt;

        // Will the ball leave the window on the x-axis? If so, revert speed on x-axis.
        let leaving_on_left_side: bool = self.position.0 + progress_x < 0.0;
        let leaving_on_right_side: bool = self.position.0 + self.diameter + progress_x > width as f64;
        if leaving_on_left_side || leaving_on_right_side {
            self.speed.0 *= -1.0;
        }

        // Will the ball leave the window on the y-axis? If so, revert speed on y-axis.
        let leaving_on_top: bool = self.position.1 + progress_y < 0.0;
        let leaving_on_bottom: bool = self.position.1 + self.diameter + progress_y > height as f64;
        if leaving_on_top || leaving_on_bottom {
            self.speed.1 *= -1.0;
        }

        // Move the ball to the new position.
        self.position = (self.position.0 + self.speed.0 * dt,
                         self.position.1 + self.speed.1 * dt);

        // Ensure the ball is entirely within the window.
        if self.position.0 < 0.0 {
            self.position.0 = 0.0;
        } else if self.position.0 + self.diameter > width as f64 {
            self.position.0 = (width as f64) - self.diameter;
        }
        if self.position.1 < 0.0 {
            self.position.1 = 0.0;
        } else if self.position.1 + self.diameter > height as f64 {
            self.position.1 = (height as f64) - self.diameter;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ball = Ball::new();
        assert_eq!(ball.diameter, 10.0);
        assert_eq!(ball.position, (0.0, 0.0));
        assert_eq!(ball.speed, (100.0, 100.0));
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn update(dt: f64, width: u32, height: u32) -> bool {
            let mut ball = Ball::new();
            ball.update(dt, width, height);

            // The window has a minimum size.
            if (width as f64) < ball.diameter || (height as f64) < ball.diameter {
                return true;
            }

            let not_leaving_on_left_side: bool = ball.position.0 >= 0.0;
            let not_leaving_on_right_side: bool = ball.position.0 + ball.diameter <= width as f64;
            let not_leaving_on_top: bool = ball.position.1 >= 0.0;
            let not_leaving_on_bottom: bool = ball.position.1 + ball.diameter <= height as f64;

            not_leaving_on_left_side &&
                not_leaving_on_right_side &&
                not_leaving_on_top &&
                not_leaving_on_bottom
        }
    }
}
