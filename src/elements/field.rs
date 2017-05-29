// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The playing field of the game.

use piston_window::Button;
use piston_window::Context;
use piston_window::G2d;
use piston_window::Key;
use piston_window::Line;
use piston_window::Transformed;
use piston_window::UpdateArgs;

use color;
use elements::Ball;
use elements::BallStatus;
use elements::FieldSide;
use elements::Movement;
use elements::Player;

/// The interval at which the ball's and the players' speeds are changed.
const SPEED_CHANGE_INTERVAL: f64 = 10.0;

/// The amount by which the speeds of the ball and players are changed.
const SPEED_CHANGE: f64 = 10.0;

/// The field where the game actually occurs.
pub struct Field {
    /// The ball used for playing.
    ball: Ball,

    /// The Δt since the last speed change.
    last_speed_change: f64,

    /// The players.
    players: [Player; 2],

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
            last_speed_change: 0.0,
            players: [
                Player::new(FieldSide::Left, size[0]),
                Player::new(FieldSide::Right, size[0])
            ],
            height: size[1],
            width: size[0],
        }
    }

    /// Get the scores of all players.
    pub fn get_player_scores(&self) -> [isize; 2] {
        [
            self.players[0].get_score(),
            self.players[1].get_score(),
        ]
    }

    /// Handle button press events.
    pub fn on_button_pressed(&mut self, button: Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::W => self.players[0].set_movement(Movement::Up),
                Key::S => self.players[0].set_movement(Movement::Down),
                Key::Up => self.players[1].set_movement(Movement::Up),
                Key::Down => self.players[1].set_movement(Movement::Down),
                _ => {},
            }
        }
    }

    /// Handle button release events.
    pub fn on_button_released(&mut self, button: Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::W | Key::S => self.players[0].set_movement(Movement::None),
                Key::Up | Key::Down => self.players[1].set_movement(Movement::None),
                _ => {},
            }
        }
    }

    /// Draw the field with its contents.
    pub fn on_render(&self, context: Context, graphics: &mut G2d) {
        let line_width: f64 = 1.0;

        // Draw the center line.
        let center_line = Line::new(color::GRAY, line_width);
        let position_x: f64 = (self.width as f64) / 2.0 - line_width;
        let number_of_dashes: u32 = 10;
        let height: f64 = (self.height as f64) / ((number_of_dashes as f64) * 2.0 - 1.0);
        for i in 0..number_of_dashes {
            let position_y: f64 = (i as f64) * height * 2.0;
            let transformation = context.transform.trans(position_x, position_y);
            center_line.draw([0.0, 0.0, 0.0, height], &context.draw_state, transformation, graphics);
        }

        // Draw the top line.
        let line = Line::new(color::WHITE, line_width);
        let transformation = context.transform.trans(0.0, 0.0 + line_width);
        line.draw([0.0, 0.0, self.width as f64, 0.0], &context.draw_state, transformation, graphics);

        // Draw the players.
        for player in &self.players {
            player.draw(&context, graphics);
        }

        // Draw the ball.
        self.ball.draw(&context, graphics);
    }

    /// Resize the field.
    pub fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        for player in &mut self.players {
            player.update_position(new_width);
        }
    }

    /// Update the field state.
    pub fn on_update(&mut self, update_arguments: &UpdateArgs) {
        let dt: f64 = update_arguments.dt;

        // Update the speeds if necessary.
        self.last_speed_change += dt;
        if self.last_speed_change >= SPEED_CHANGE_INTERVAL {
            self.last_speed_change = 0.0;

            self.ball.change_speed(SPEED_CHANGE);
            for player in &mut self.players {
                player.change_speed(SPEED_CHANGE);
            }
        }

        self.players[0].update(dt, self.height);
        self.players[1].update(dt, self.height);

        let player_handles = [
            self.players[0].get_bounding_box(),
            self.players[1].get_bounding_box(),
        ];

        let status: BallStatus = self.ball.update(dt, self.width, self.height, &player_handles);
        self.update_scores(status);
    }

    /// If the ball left the field on the left or right side, the other side's player will get a point.
    fn update_scores(&mut self, status: BallStatus) {
        match status {
            BallStatus::WithinGame => return,
            BallStatus::LeftOnLeftSide => {
                self.players[1].update_score(1);
            },
            BallStatus::LeftOnRightSide => {
                self.players[0].update_score(1);
            }
        }

        // The ball left the field. Create a new one.
        self.ball = Ball::new([self.width, self.height]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let field = Field::new([200, 100]);
        assert!(field.last_speed_change <= 0.0);  // The first speed change might take longer to happen.
        assert_eq!(field.width, 200);
        assert_eq!(field.height, 100);
    }

    #[test]
    fn get_player_scores() {
        let mut field = Field::new([200, 100]);
        field.players[0].update_score(42);
        field.players[1].update_score(-42);
        let scores: [isize; 2] = field.get_player_scores();
        assert_eq!(scores, [42, -42]);
    }

    #[test]
    fn on_resize() {
        let mut field = Field::new([200, 100]);
        field.on_resize(100, 200);
        assert_eq!(field.width, 100);
        assert_eq!(field.height, 200);
    }
}
