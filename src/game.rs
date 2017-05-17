// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! This module contains the game management logic.

use opengl_graphics::GlGraphics;
use piston_window::clear;
use piston_window::OpenGL;
use piston_window::PistonWindow;
use piston_window::RenderArgs;
use piston_window::RenderEvent;
use piston_window::WindowSettings;

use error::Result;
use color;

/// The OpenGL version.
const OPENGL: OpenGL = OpenGL::V3_2;

/// A struct managing the game logic.
pub struct Game {
    /// The game window.
    window: PistonWindow,

    /// OpenGL data.
    gl_graphics: GlGraphics,
}

impl Game {
    /// Initialize a new game instance.
    pub fn new() -> Result<Game> {
        let window: PistonWindow = WindowSettings::new("Mief", [800, 480])
            .opengl(OPENGL)
            .exit_on_esc(true)
            .build()?;

        Ok(Game {
            gl_graphics: GlGraphics::new(OPENGL),
            window: window,
        })
    }

    /// Render the entire game.
    fn render(&mut self, render_arguments: &RenderArgs) {
        self.gl_graphics.draw(render_arguments.viewport(), |_context, gl_graphics| {
            clear(color::BLACK, gl_graphics);
        });
    }

    /// Run the game.
    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            if let Some(render_arguments) = event.render_args() {
                self.render(&render_arguments);
            }
        }
    }
}
