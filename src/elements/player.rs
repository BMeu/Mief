// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The player and directly associated structures.

use piston_window::Context;
use piston_window::G2d;
use piston_window::Rectangle;
use piston_window::Transformed;
#[cfg(test)]
use quickcheck::Arbitrary;
#[cfg(test)]
use quickcheck::Gen;

use color;

/// The margin between the player's handle and the respective edge of the field.
const PLAYER_MARGIN: f64 = 10.0;

/// The player's initial speed.
const SPEED: f64 = 150.0;

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
            1 => Movement::Up,
            _ => Movement::None,
        }
    }
}

/// The player's position on the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldSide {
    /// The player plays on the left side of the field.
    Left,

    /// The player plays on the right side of the field.
    Right,
}

impl FieldSide {
    /// Get the x position on the field, depending on the field size.
    pub fn get_x_position(&self, player_width: f64, field_width: u32) -> f64 {
        match *self {
            FieldSide::Left => PLAYER_MARGIN,
            FieldSide::Right => (field_width as f64) - player_width - PLAYER_MARGIN,
        }
    }
}

/// The player.
#[derive(Clone, Copy, Debug)]
pub struct Player {
    /// The player's position on the field.
    field_side: FieldSide,

    /// The current direction of movement.
    movement: Movement,

    /// The current position of the player: `(x, y)`.
    position: (f64, f64),

    /// The points the player achieved so far.
    score: isize,

    /// The size of the player's handle: `(width, height)`.
    size: (f64, f64),

    /// The current speed of the player (the player can only move in `y`-direction).
    speed: f64,
}

impl Player {
    /// Create a new player at position `(x, y)`.
    pub fn new(side: FieldSide, field_width: u32) -> Player {
        let size: (f64, f64) = (10.0, 60.0);
        let y: f64 = 0.0;
        let x: f64 = side.get_x_position(size.0, field_width);

        Player {
            field_side: side,
            movement: Movement::None,
            position: (x, y),
            score: 0,
            size: (10.0, 60.0),
            speed: SPEED,
        }
    }

    /// Change the player's speed by the given `amount`.
    pub fn change_speed(&mut self, amount: f64) {
        self.speed += amount;
    }

    /// Draw the player.
    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        let handle = Rectangle::new(color::WHITE);
        let transformation = context.transform.trans(self.position.0, self.position.1);
        handle.draw([0.0, 0.0, self.size.0, self.size.1], &context.draw_state, transformation, graphics);
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

    /// Get the player's current score.
    pub fn get_score(&self) -> isize {
        self.score
    }

    /// Move the player.
    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
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

        // Reset the speed.
        self.speed = SPEED;
    }

    /// Update the player's position depending on the new width of the field.
    pub fn update_position(&mut self, new_field_width: u32) {
        self.position.0 = self.field_side.get_x_position(self.size.0, new_field_width);
    }
}

#[cfg(test)]
mod tests {
    #![allow(trivial_casts)]

    use quickcheck::TestResult;
    use super::*;

    #[test]
    fn get_x_position_left() {
        let side = FieldSide::Left;
        let x: f64 = side.get_x_position(20.0, 50);
        assert_eq!(x, PLAYER_MARGIN);
    }

    #[test]
    fn get_x_position_right() {
        let side = FieldSide::Right;
        let x: f64 = side.get_x_position(20.0, 50);
        assert_eq!(x, 30.0 - PLAYER_MARGIN);
    }

    #[test]
    fn new() {
        let player = Player::new(FieldSide::Left, 42);
        assert_eq!(player.movement, Movement::None);
        assert_eq!(player.position, (PLAYER_MARGIN, 0.0));
        assert_eq!(player.score, 0);
        assert_eq!(player.size, (10.0, 60.0));
        assert_eq!(player.speed, 150.0);
    }

    #[test]
    fn change_speed() {
        let mut player = Player::new(FieldSide::Left, 42);
        player.speed = 42.0;
        player.change_speed(10.0);
        assert_eq!(player.speed, 52.0);
    }

    #[test]
    fn get_bounding_box() {
        let player = Player::new(FieldSide::Left, 42);
        let bounding_box = player.get_bounding_box();
        assert_eq!(bounding_box[0], PLAYER_MARGIN);
        assert_eq!(bounding_box[1], 0.0);
        assert_eq!(bounding_box[2], PLAYER_MARGIN + 10.0);
        assert_eq!(bounding_box[3], 60.0);
    }

    #[test]
    fn get_score() {
        let mut player = Player::new(FieldSide::Left, 42);
        let score: isize = 42;
        player.score = score;
        assert_eq!(player.get_score(), score);
    }

    quickcheck! {
        fn set_movement(movement: Movement) -> bool {
        let mut player = Player::new(FieldSide::Left, 42);
        player.set_movement(movement);

        player.movement == movement
        }
    }

    quickcheck! {
        fn update(position: (f64, f64), dt: f64, height: u32, movement: Movement) -> TestResult {
            // Time only advances, the position cannot be negative.
            if dt.is_sign_negative() || position.0.is_sign_negative() || position.1.is_sign_negative() {
                return TestResult::discard();
            }

            let mut player = Player::new(FieldSide::Left, (position.1 * 2.0) as u32);
            player.position = position;
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
        fn update_score(old_score: isize, additional_points: isize, speed: f64) -> bool {
            let mut player = Player::new(FieldSide::Left, 42);
            player.speed = speed;
            player.score = old_score;
            player.update_score(additional_points);

            if additional_points == 0 {
                // If the additional points are 0, the score must not change.
                return player.score == old_score &&
                    player.speed == SPEED;
            } else if additional_points > 0 {
                // If the additional points are positive, the new score must be greater than the old score, but must not
                // overflow.
                return old_score < player.score && player.score <= ::std::isize::MAX &&
                    player.speed == SPEED;
            } else {
                // If the additional points are negative, the new score must be smaller than the old score, but must not
                // overflow.
                return ::std::isize::MIN <= player.score && player.score < old_score &&
                    player.speed == SPEED;
            }
        }
    }

    #[test]
    fn update_score_upper_overflow() {
        let mut player = Player::new(FieldSide::Left, 42);
        player.score = ::std::isize::MAX;
        player.update_score(1);
        assert_eq!(player.score, ::std::isize::MAX);
    }

    #[test]
    fn update_score_lower_overflow() {
        let mut player = Player::new(FieldSide::Left, 42);
        player.score = ::std::isize::MIN;
        player.update_score(-1);
        assert_eq!(player.score, ::std::isize::MIN);
    }

    #[test]
    fn update_position() {
        let mut player = Player::new(FieldSide::Right, 42);
        player.update_position(60);
        assert_eq!(player.position, (50.0 - PLAYER_MARGIN, 0.0));
    }
}
