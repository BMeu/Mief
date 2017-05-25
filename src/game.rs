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
use piston_window::UpdateArgs;
use piston_window::WindowSettings;

use elements::Field;
use execution_flow::Result;
use color;

/// The OpenGL version.
const OPENGL: OpenGL = OpenGL::V3_2;

/// A struct managing the game logic.
pub struct Game {
    /// The game window.
    window: PistonWindow,

    /// The playing field.
    field: Field,
}

impl Game {
    /// Initialize a new game instance.
    pub fn new() -> Result<Game> {
        let window_size: [u32; 2] = [800, 480];

        let window: PistonWindow = WindowSettings::new("Mief", window_size)
            .opengl(OPENGL)
            .exit_on_esc(true)
            .build()?;

        Ok(Game {
            window: window,
            field: Field::new(window_size),
        })
    }

    /// Render the entire game.
    fn draw(&mut self, event: &Input) {
        let field: &mut Field = &mut self.field;

        let _ = self.window.draw_2d(event, |context, gl_graphics| {
            clear(color::BLACK, gl_graphics);

            field.draw(&context, gl_graphics);
        });
    }

    /// Resize the entire game.
    fn resize(&mut self, width: u32, height: u32) {
        self.field.resize(width, height);
    }

    /// Update the game state.
    fn update(&mut self, update_arguments: &UpdateArgs) {
        self.field.update(update_arguments);
    }

    /// Run the game.
    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            match event {
                Input::Render(_) => self.draw(&event),
                Input::Resize(width, height) => self.resize(width, height),
                Input::Update(update_arguments) => self.update(&update_arguments),
                _ => {},
            }
        }
    }
}
