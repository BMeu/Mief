// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The ball used for playing and directly associated structures.

use piston_window::Context;
use piston_window::Ellipse;
use piston_window::G2d;
use piston_window::Transformed;
use rand::thread_rng;
use rand::Rng;
use rand::ThreadRng;

use color;

/// The current status of the ball.
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BallStatus {
    /// The ball left the field on the left side.
    LeftOnLeftSide,

    /// The ball left the field on the right side.
    LeftOnRightSide,

    /// The ball is still within the field.
    WithinGame,
}

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
    pub fn update(&mut self, dt: f64, width: u32, height: u32, obstacles: &[[f64; 4]]) -> BallStatus {
        let progress_x = self.speed.0 * dt;
        let progress_y = self.speed.1 * dt;
        let next_position: (f64, f64) = (self.position.0 + progress_x, self.position.1 + progress_y);

        // Check for collisions with any obstacles.
        for obstacle in obstacles {
            self.collide_with(next_position, obstacle);
        }

        // Will the ball leave the window on the x-axis? If so, it is a point for the other side's player.
        let leaving_on_left_side: bool = self.position.0 + progress_x < 0.0;
        if leaving_on_left_side {
            return BallStatus::LeftOnLeftSide;
        }
        let leaving_on_right_side: bool = self.position.0 + self.diameter + progress_x > width as f64;
        if leaving_on_right_side {
            return BallStatus::LeftOnRightSide;
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

        // Ensure the ball is entirely within the window on the y-axis.
        if self.position.1 < 0.0 {
            self.position.1 = 0.0;
        } else if self.position.1 + self.diameter > height as f64 {
            self.position.1 = (height as f64) - self.diameter;
        }

        BallStatus::WithinGame
    }

    /// Check if the ball will collide with `object`'s bounding box at `next_position` and reverse the ball's
    /// direction accordingly.
    fn collide_with(&mut self, next_position: (f64, f64), object: &[f64; 4]) {
        let radius: f64 = self.diameter / 2.0;
        let (x, y): (f64, f64) = next_position;

        // Use more obvious names for the other object's position.
        let (left_x, top_y, right_x, bottom_y) = (object[0], object[1], object[2], object[3]);

        // Did the ball hit the object from the top or bottom?
        let hit_horizontal_edge: bool =
            x + radius >= left_x &&         // The ball must be within the other object's width.
            x + radius <= right_x &&
            y + self.diameter >= top_y &&   // The ball must not be above the object.
            y <= bottom_y;                  // The ball must not be below the object.
        if hit_horizontal_edge {
            self.speed.1 *= -1.0;
        }

        // Did the ball hit the object on the left or right side?
        let hit_lateral_edge: bool =
            y + radius >= top_y &&          // The ball must be within the other object's height.
            y + radius <= bottom_y &&
            x + self.diameter >= left_x &&  // The ball must not be to the left of the object.
            x <= right_x;                   // The ball must not be to the right of the object.
        if hit_lateral_edge {
            self.speed.0 *= -1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(trivial_casts)]

    use quickcheck::TestResult;
    use super::*;

    /// Two `f64` numbers are equal iff their difference is within `std::f64::EPSILON`.
    fn approx_eq(first: f64, second: f64) -> bool {
        first - second <= ::std::f64::EPSILON
    }

    quickcheck! {
        fn new(width: u32, height: u32) -> TestResult {
            let ball = Ball::new([width, height]);
            assert_eq!(ball.diameter, 10.0);

            // The window has a minimum size.
            if (width as f64) < ball.diameter || (height as f64) < ball.diameter {
                return TestResult::discard();
            }

            // The margins of the ball must be the same on each axis.
            let left_equals_right_margin: bool = approx_eq(ball.position.0,
                                                           (width as f64) - ball.position.0 + ball.diameter);
            let top_equals_bottom_margin: bool = approx_eq(ball.position.1,
                                                           (height as f64) - ball.position.1 + ball.diameter);

            // The (absolute) speed in either direction should be between 100 and 150.
            let speed_x: f64 = ball.speed.0.abs();
            let speed_y: f64 = ball.speed.1.abs();
            let is_valid_speed_x: bool = 100.0 <= speed_x && speed_x <= 150.0;
            let is_valid_speed_y: bool = 100.0 <= speed_y && speed_y <= 150.0;

            TestResult::from_bool(
                left_equals_right_margin &&
                top_equals_bottom_margin &&
                is_valid_speed_x &&
                is_valid_speed_y
            )
        }
    }

    #[test]
    fn update_no_collision() {
        let (width, height): (u32, u32) = (100, 100);
        let speed: (f64, f64) = (100.0, 100.0);
        let mut ball = Ball::new([width, height]);
        ball.speed = speed;
        assert_eq!(ball.position, (45.0, 45.0));

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, speed);
        assert_eq!(ball.position, (55.0, 55.0));
    }

    #[test]
    fn update_reflect_on_top() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (100.0, -100.0);
        ball.position = (45.0, 5.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, (100.0, 100.0));
        assert_eq!(ball.position, (55.0, 15.0));
    }

    #[test]
    fn update_reflect_on_bottom() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (100.0, 100.0);
        ball.position = (45.0, 95.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, (100.0, -100.0));
        assert_eq!(ball.position, (55.0, 85.0));
    }

    #[test]
    fn update_reposition_to_top() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (100.0, -100.0);
        ball.position = (45.0, -15.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, (100.0, 100.0));
        assert_eq!(ball.position, (55.0, 0.0));
    }

    #[test]
    fn update_reposition_to_bottom() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (100.0, 100.0);
        ball.position = (45.0, 110.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, (100.0, -100.0));
        assert_eq!(ball.position, (55.0, 90.0));
    }

    #[test]
    fn update_leave_on_left() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (-100.0, 100.0);
        ball.position = (5.0, 45.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::LeftOnLeftSide);
        assert_eq!(ball.speed, (-100.0, 100.0));
        assert_eq!(ball.position, (5.0, 45.0));
    }

    #[test]
    fn update_leave_on_right() {
        let (width, height): (u32, u32) = (100, 100);
        let mut ball = Ball::new([width, height]);
        ball.speed = (100.0, 100.0);
        ball.position = (95.0, 45.0);

        let status = ball.update(0.1, width, height, &[]);
        assert_eq!(status, BallStatus::LeftOnRightSide);
        assert_eq!(ball.speed, (100.0, 100.0));
        assert_eq!(ball.position, (95.0, 45.0));
    }

    #[test]
    fn update_collide() {
        let (width, height): (u32, u32) = (100, 100);
        let object: [f64; 4] = [45.0, 45.0, 55.0, 55.0];
        let mut ball = Ball::new([width, height]);
        ball.speed = (-100.0, 100.0);
        ball.position = (65.0, 40.0);

        let status = ball.update(0.1, width, height, &[object]);
        assert_eq!(status, BallStatus::WithinGame);
        assert_eq!(ball.speed, (100.0, 100.0));
        assert_eq!(ball.position, (75.0, 50.0));
    }

    #[test]
    fn collide_with_no_collision() {
        let mut ball = Ball::new([100, 100]);
        let old_speed: (f64, f64) = ball.speed;
        let object: [f64; 4] = [75.0, 75.0, 85.0, 85.0];

        ball.collide_with((25.0, 25.0), &object);
        assert_eq!(ball.speed, old_speed);
    }

    #[test]
    fn collide_with_on_top() {
        let mut ball = Ball::new([100, 100]);
        let old_speed: (f64, f64) = ball.speed;
        let object: [f64; 4] = [75.0, 75.0, 85.0, 85.0];

        ball.collide_with((80.0, 65.0), &object);
        assert_eq!(ball.speed, (old_speed.0, old_speed.1 * -1.0));
    }

    #[test]
    fn collide_with_on_right() {
        let mut ball = Ball::new([100, 100]);
        let old_speed: (f64, f64) = ball.speed;
        let object: [f64; 4] = [75.0, 75.0, 85.0, 85.0];

        ball.collide_with((85.0, 80.0), &object);
        assert_eq!(ball.speed, (old_speed.0 * -1.0, old_speed.1));
    }

    #[test]
    fn collide_with_on_bottom() {
        let mut ball = Ball::new([100, 100]);
        let old_speed: (f64, f64) = ball.speed;
        let object: [f64; 4] = [75.0, 75.0, 85.0, 85.0];

        ball.collide_with((80.0, 85.0), &object);
        assert_eq!(ball.speed, (old_speed.0, old_speed.1 * -1.0));
    }

    #[test]
    fn collide_with_on_left() {
        let mut ball = Ball::new([100, 100]);
        let old_speed: (f64, f64) = ball.speed;
        let object: [f64; 4] = [75.0, 75.0, 85.0, 85.0];

        ball.collide_with((65.0, 80.0), &object);
        assert_eq!(ball.speed, (old_speed.0 * -1.0, old_speed.1));
    }
}
