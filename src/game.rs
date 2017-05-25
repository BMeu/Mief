// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the game management logic.

use piston_window::clear;
use piston_window::Input;
use piston_window::OpenGL;
use piston_window::PistonWindow;
use piston_window::RenderEvent;
use piston_window::UpdateArgs;
use piston_window::UpdateEvent;
use piston_window::Window;
use piston_window::WindowSettings;

use elements::Ball;
use execution_flow::Result;
use color;

/// The OpenGL version.
const OPENGL: OpenGL = OpenGL::V3_2;

/// A struct managing the game logic.
pub struct Game {
    /// The game window.
    window: PistonWindow,

    /// Ball.
    ball: Ball,
}

impl Game {
    /// Initialize a new game instance.
    pub fn new() -> Result<Game> {
        let window: PistonWindow = WindowSettings::new("Mief", [800, 480])
            .opengl(OPENGL)
            .exit_on_esc(true)
            .build()?;

        Ok(Game {
            window: window,
            ball: Ball::new(),
        })
    }

    /// Render the entire game.
    fn render(&mut self, event: &Input) {
        let ball: &mut Ball = &mut self.ball;

        let _ = self.window.draw_2d(event, |context, gl_graphics| {
            clear(color::BLACK, gl_graphics);

            ball.draw(&context, gl_graphics);
        });
    }

    /// Update the game state.
    fn update(&mut self, update_arguments: &UpdateArgs) {
        self.ball.update(update_arguments.dt, self.window.size().width, self.window.size().height);
    }

    /// Run the game.
    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            if let Some(_render_arguments) = event.render_args() {
                self.render(&event);
            }

            if let Some(update_arguments) = event.update_args() {
                self.update(&update_arguments);
            }
        }
    }
}
