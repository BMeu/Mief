// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The highest abstraction of the application logic, including window creation.

use std::path::PathBuf;

use find_folder::Search;
use piston_window::clear;
use piston_window::Button;
use piston_window::Glyphs;
use piston_window::Input;
use piston_window::OpenGL;
use piston_window::PistonWindow;
use piston_window::RenderArgs;
use piston_window::Transformed;
use piston_window::UpdateArgs;
use piston_window::WindowSettings;
use piston_window::character::CharacterCache;
use piston_window::text::Text;

use elements::Field;
use execution_flow::Result;
use color;

/// The OpenGL version.
const OPENGL: OpenGL = OpenGL::V3_2;

/// The manager of the application logic.
pub struct Application {
    /// Path to the folder containing the assets.
    assets: PathBuf,

    /// The application window.
    window: PistonWindow,

    /// The playing field.
    field: Field,
}

impl Application {
    /// Initialize a new application.
    ///
    /// Returns an error if the `PistonWindow` cannot be initialized.
    pub fn new() -> Result<Application> {
        let window_size: [u32; 2] = [800, 600];

        let window: PistonWindow = WindowSettings::new("Mief", window_size)
            .opengl(OPENGL)
            .exit_on_esc(true)
            .resizable(false)  // Not yet working - see https://github.com/PistonDevelopers/piston_window/issues/160.
            .build()?;

        let assets: PathBuf = Search::ParentsThenKids(3, 1).for_folder("assets")?;

        Ok(Application {
            assets: assets,
            window: window,
            field: Field::new([800, 480]),
        })
    }

    /// Handle button press events.
    fn on_button_pressed(&mut self, button: Button) {
        self.field.on_button_pressed(button);
    }

    /// Handle button release events.
    fn on_button_released(&mut self, button: Button) {
        self.field.on_button_released(button);
    }

    /// Render the entire application.
    fn on_render(&mut self, event: &Input, render_arguments: &RenderArgs) {
        let font: PathBuf = self.assets.join("Anonymous Pro.ttf");
        let factory = self.window.factory.clone();
        let mut font = Glyphs::new(font, factory).unwrap();
        let font_size: u32 = 60;

        let field: &mut Field = &mut self.field;

        let _ = self.window.draw_2d(event, |context, gl_graphics| {
            clear(color::BLACK, gl_graphics);

            field.on_render(context.trans(0.0, 120.0), gl_graphics);

            // Draw the title.
            let title_text: &str = "Mief";
            let title_width: f64 = font.width(font_size, title_text);
            let title = Text::new_color(color::WHITE, font_size);
            let transformation = context.transform.trans(((render_arguments.width as f64) - title_width) / 2.0, 90.0);
            title.draw("Mief", &mut font, &context.draw_state, transformation, gl_graphics);

            // Get the scores.
            let scores: [isize; 2] = field.get_player_scores();
            let left_score: &str = &scores[0].to_string();
            let right_score: &str = &scores[1].to_string();

            // Draw the left player's score.
            let score = Text::new_color(color::WHITE, font_size);
            let transformation = context.transform.trans(10.0, 90.0);
            score.draw(left_score, &mut font, &context.draw_state, transformation, gl_graphics);

            // Draw the right player's score.
            let right_score_width: f64 = font.width(font_size, right_score);
            let score = Text::new_color(color::WHITE, font_size);
            let transformation = context.transform.trans((render_arguments.width as f64) - right_score_width - 10.0, 90.0);
            score.draw(right_score, &mut font, &context.draw_state, transformation, gl_graphics);
        });
    }

    /// Update the application state.
    fn on_update(&mut self, update_arguments: &UpdateArgs) {
        self.field.on_update(update_arguments);
    }

    /// Run the application.
    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            match event {
                Input::Press(button) => self.on_button_pressed(button),
                Input::Release(button) => self.on_button_released(button),
                Input::Render(render_arguments) => self.on_render(&event, &render_arguments),
                Input::Update(update_arguments) => self.on_update(&update_arguments),
                _ => {},
            }
        }
    }
}
