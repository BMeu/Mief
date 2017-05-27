// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the player.
//!
use piston_window::Context;
use piston_window::G2d;
use piston_window::Rectangle;
use piston_window::Transformed;
#[cfg(test)]
use quickcheck::Arbitrary;
#[cfg(test)]
use quickcheck::Gen;

use color;

/// The direction of the player's movement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Movement {
    /// Move the handle down.
    Down,

    /// Do not move.
    None,

    /// Move the handle up.
    Up,
}

#[cfg(test)]
impl Arbitrary for Movement {
    /// Implement the `Arbitrary` trait so this enum can be used in `quickcheck` tests.
    fn arbitrary<G: Gen>(g: &mut G) -> Movement {
        let mode = g.gen_range(0, 3);
        match mode {
            0 => Movement::Down,
            1 => Movement::None,
            2 => Movement::Up,
            _ => unreachable!()
        }
    }
}

/// The player.
#[derive(Clone, Copy, Debug)]
pub struct Player {
    /// The current direction of movement.
    movement: Movement,

    /// The current position of the player: `(x, y)`.
    position: (f64, f64),

    /// The points the player achieved.
    score: isize,

    /// The size of the player's handle: `(width, height)`.
    size: (f64, f64),

    /// The current speed of the player (the player can only move in y-direction).
    speed: f64,
}

impl Player {
    /// Crate a new player at position `(x, y)`.
    pub fn new(position: (f64, f64)) -> Player {
        Player {
            movement: Movement::None,
            position: position,
            score: 0,
            size: (10.0, 60.0),
            speed: 150.0,
        }
    }

    /// Draw the player.
    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        let handle = Rectangle::new(color::WHITE);
        let transformation = context.transform.trans(self.position.0, self.position.1);
        handle.draw([0.0, 0.0, self.size.0, self.size.1], &context.draw_state, transformation, graphics);
    }

    /// Update the player's position.
    pub fn update(&mut self, dt: f64, height: u32) {
        match self.movement {
            Movement::Down => {
                self.position.1 += self.speed * dt;
                if self.position.1 + self.size.1 > (height as f64) {
                    self.position.1 = (height as f64) - self.size.1;
                }
            },
            Movement::Up => {
                self.position.1 -= self.speed * dt;
                if self.position.1 < 0.0 {
                    self.position.1 = 0.0;
                }
            },
            _ => {},
        }
    }

    /// Move the player.
    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }

    /// Update the player's score with `additional_points`.
    ///
    /// If the new score would overflow (in either direction), the score is set to `isize::MAX` or `isize::MIN`,
    /// respectively.
    pub fn update_score(&mut self, additional_points: isize) {
        // Do not let the player cheat by preventing overflows in either direction.
        match self.score.checked_add(additional_points) {
            Some(new_score) => self.score = new_score,
            None => {
                if additional_points >= 0 {
                    self.score = ::std::isize::MAX;
                }
                else {
                    self.score = ::std::isize::MIN;
                }
            }
        }
    }

    /// Get the bounding box of the player's handle.
    #[inline]
    pub fn get_bounding_box(&self) -> [f64; 4] {
        [
            self.position.0,                // Left x.
            self.position.1,                // Top y.
            self.position.0 + self.size.0,  // Right x.
            self.position.1 + self.size.1   // Bottom y.
        ]
    }
}

#[cfg(test)]
mod tests {
    #![allow(trivial_casts)]

    use quickcheck::TestResult;
    use super::*;

    #[test]
    fn new() {
        let player = Player::new((42.0, 13.37));
        assert_eq!(player.movement, Movement::None);
        assert_eq!(player.position, (42.0, 13.37));
        assert_eq!(player.score, 0);
        assert_eq!(player.size, (10.0, 60.0));
        assert_eq!(player.speed, 150.0);
    }

    quickcheck! {
        fn update(position: (f64, f64), dt: f64, height: u32, movement: Movement) -> TestResult {
            // Time only advances, the position cannot be negative.
            if dt.is_sign_negative() || position.0.is_sign_negative() || position.1.is_sign_negative() {
                return TestResult::discard();
            }

            let mut player = Player::new(position);
            player.set_movement(movement);
            player.update(dt, height);

            // The field's height must be at least as big as the player's height plus the current y-position.
            if (height as f64) < position.1 + player.size.1 {
                return TestResult::discard();
            }

            match movement {
                Movement::None => {
                    // Without any movement, the player's position must not change.
                    TestResult::from_bool(player.position == position)
                },
                Movement::Up => {
                    TestResult::from_bool(
                        player.position.0 == position.0 &&  // The x-position must not change.
                        player.position.1 >= 0.0 &&         // The handle must be within the field.
                        player.position.1 <= position.1     // The handle must move upwards.
                    )
                },
                Movement::Down => {
                    TestResult::from_bool(
                        player.position.0 == position.0 &&                    // The x-position must not change.
                        player.position.1 >= position.1 &&                    // The handle must move downwards.
                        player.position.1 + player.size.1 <= (height as f64)  // The handle must be within the field.
                    )
                }
            }
        }
    }

    quickcheck! {
        fn set_movement(movement: Movement) -> bool {
        let mut player = Player::new((0.0, 0.0));
        player.set_movement(movement);

        player.movement == movement
        }
    }

    quickcheck! {
        fn update_score(old_score: isize, additional_points: isize) -> bool {
            let mut player = Player::new((0.0, 0.0));
            player.score = old_score;
            player.update_score(additional_points);

            if additional_points == 0 {
                // If the additional points are 0, the score must not change.
                return player.score == old_score;
            } else if additional_points > 0 {
                // If the additional points are positive, the new score must be greater than the old score, but must not
                // overflow.
                return old_score < player.score && player.score <= ::std::isize::MAX;
            } else {
                // If the additional points are negative, the new score must be smaller than the old score, but must not
                // overflow.
                return ::std::isize::MIN <= player.score && player.score < old_score;
            }
        }
    }

    #[test]
    fn get_bounding_box() {
        let player = Player::new((42.0, 13.37));
        let bounding_box = player.get_bounding_box();
        assert_eq!(bounding_box[0], 42.0);
        assert_eq!(bounding_box[1], 13.37);
        assert_eq!(bounding_box[2], 52.0);
        assert_eq!(bounding_box[3], 73.37);
    }
}
