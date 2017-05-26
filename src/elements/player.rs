// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the player.
//!
use piston_window::Context;
use piston_window::Rectangle;
use piston_window::G2d;
use piston_window::Transformed;

use color;

/// The direction of the player's movement.
#[derive(Clone, Copy, Debug)]
pub enum Movement {
    /// Move the handle down.
    Down,

    /// Do not move.
    None,

    /// Move the handle up.
    Up,
}

/// The player.
#[derive(Clone, Copy, Debug)]
pub struct Player {
    /// The current direction of movement.
    movement: Movement,

    /// The current position of the player: `(x, y)`.
    position: (f64, f64),

    /// The points the player achieved.
    score: usize,

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

    /// Get the bounding box of the player's handle.
    #[inline]
    pub fn get_bounding_box(&self) -> [f64; 4] {
        [
            self.position.0,
            self.position.1,
            self.position.0 + self.size.0,
            self.position.1 + self.size.1
        ]
    }
}
