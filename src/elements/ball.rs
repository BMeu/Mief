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
use rand::thread_rng;
use rand::Rng;
use rand::ThreadRng;

use color;
use elements::Player;

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
    /// Create a new ball with a random speed at the center of the window (given by `[width, height]`).
    pub fn new(window_size: [u32; 2]) -> Ball {
        let width = window_size[0] as f64;
        let height = window_size[1] as f64;

        let radius: f64 = 5.0;
        let mut position: (f64, f64) = (width / 2.0 - radius, height / 2.0 - radius);
        if position.0 < 0.0 {
            position.0 = 0.0;
        }
        if position.1 < 0.0 {
            position.1 = 0.0;
        }

        // Randomly choose the speed.
        let mininum_speed: f64 = 100.0;
        let maximum_speed: f64 = 150.0;
        let mut rng: ThreadRng = thread_rng();
        let mut speed_x: f64 = rng.gen_range(mininum_speed, maximum_speed);
        if rng.gen::<bool>() {
            speed_x *= -1.0;
        }
        let mut speed_y: f64 = rng.gen_range(mininum_speed, maximum_speed);
        if rng.gen::<bool>() {
            speed_y *= -1.0;
        }

        Ball {
            diameter: radius * 2.0,
            position: position,
            speed: (speed_x, speed_y),
        }
    }

    /// Draw the ball.
    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        let ball = Ellipse::new(color::WHITE).resolution(100);
        let transformation = context.transform.trans(self.position.0, self.position.1);
        ball.draw([0.0, 0.0, self.diameter, self.diameter], &context.draw_state, transformation, graphics);
    }

    /// Update the ball's position. `dt` is the change in time since the last update, `width` and `height` are the
    /// window's size.
    pub fn update(&mut self, dt: f64, width: u32, height: u32, player_1: Player, player_2: Player) {
        let progress_x = self.speed.0 * dt;
        let progress_y = self.speed.1 * dt;

        // Will the ball leave the window on the x-axis? If so, revert speed on x-axis.
        let leaving_on_left_side: bool = self.position.0 + progress_x < 0.0;
        let leaving_on_right_side: bool = self.position.0 + self.diameter + progress_x > width as f64;

        // TODO: Check for collisions on all sides of the handle.
        let hit_player_1: bool = self.position.0 + progress_x < player_1.get_bounding_box()[2] &&
            self.position.1 + self.diameter / 2.0  + progress_y >= player_1.get_bounding_box()[1] &&
            self.position.1 + self.diameter / 2.0  + progress_y <= player_1.get_bounding_box()[3];
        let hit_player_2: bool = self.position.0 + self.diameter + progress_x > player_2.get_bounding_box()[0] &&
            self.position.1 + self.diameter / 2.0 + progress_y >= player_2.get_bounding_box()[1] &&
            self.position.1 + self.diameter / 2.0 + progress_y  <= player_2.get_bounding_box()[3];

        if leaving_on_left_side || leaving_on_right_side || hit_player_1 || hit_player_2 {
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
    #![allow(trivial_casts)]

    use super::*;

    /// Two `f64` numbers are equal iff their difference is within `std::f64::EPSILON`.
    fn eq_epsilon(first: f64, second: f64) -> bool {
        first - second <= ::std::f64::EPSILON
    }

    quickcheck! {
        fn new(width: u32, height: u32) -> bool {
            let ball = Ball::new([width, height]);
            assert_eq!(ball.diameter, 10.0);

            // The window has a minimum size.
            if (width as f64) < ball.diameter || (height as f64) < ball.diameter {
                return true;
            }

            // The margins of the ball must be the same on each axis.
            let left_equals_right_margin: bool = eq_epsilon(ball.position.0,
                                                            (width as f64) - ball.position.0 + ball.diameter);
            let top_equals_bottom_margin: bool = eq_epsilon(ball.position.1,
                                                            (height as f64) - ball.position.1 + ball.diameter);

            // The (absolute) speed in either direction should be between 100 and 150.
            let speed_x: f64 = ball.speed.0.abs();
            let speed_y: f64 = ball.speed.1.abs();
            let is_valid_speed_x: bool = 100.0 <= speed_x && speed_x <= 150.0;
            let is_valid_speed_y: bool = 100.0 <= speed_y && speed_y <= 150.0;

            left_equals_right_margin &&
                top_equals_bottom_margin &&
                is_valid_speed_x &&
                is_valid_speed_y
        }
    }

    quickcheck! {
        fn update(dt: f64, width: u32, height: u32) -> bool {
            let mut ball = Ball::new([width, height]);
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
