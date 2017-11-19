// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The highest abstraction of the application logic, including window creation.

use std::path::PathBuf;

use find_folder::Search;
#[cfg(feature = "display-fps")]
use fps_counter::FPSCounter;
use piston_window::clear;
use piston_window::Button;
use piston_window::ButtonArgs;
use piston_window::ButtonState;
use piston_window::Event;
use piston_window::Glyphs;
use piston_window::Input;
use piston_window::Loop;
use piston_window::OpenGL;
use piston_window::PistonWindow;
use piston_window::RenderArgs;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::UpdateArgs;
use piston_window::WindowSettings;
#[cfg(feature = "display-fps")]
use piston_window::text::Text;

use elements::Field;
use elements::Scoreboard;
use execution_flow::Result;
use color;

/// The OpenGL version.
const OPENGL: OpenGL = OpenGL::V3_2;

/// The (currently) fixed height of the scoreboard.
const SCOREBOARD_HEIGHT: u32 = 120;

/// The manager of the application logic.
pub struct Application {
    /// Path to the folder containing the assets.
    assets: PathBuf,

    /// The application window.
    window: PistonWindow,

    /// The playing field.
    field: Field,

    /// The scoreboard.
    scoreboard: Scoreboard,

    /// The FPS counter.
    #[cfg(feature = "display-fps")]
    fps_counter: FPSCounter,
}

impl Application {
    /// Initialize a new application.
    ///
    /// Returns an error if the `PistonWindow` cannot be initialized.
    pub fn new() -> Result<Application> {
        let width: u32 = 800;
        let height: u32 = 600;
        let title: &str = "Mief";

        let window: PistonWindow = WindowSettings::new(title, [width, height])
            .exit_on_esc(true)
            .opengl(OPENGL)
            .resizable(false)  // Not yet working - see https://github.com/PistonDevelopers/piston_window/issues/160.
            .vsync(true)
            .build()?;

        let assets: PathBuf = Search::ParentsThenKids(3, 1).for_folder("assets")?;

        let application = match () {
            #[cfg(feature = "display-fps")]
            () => {
                Application {
                    assets: assets,
                    window: window,
                    field: Field::new([width, height - SCOREBOARD_HEIGHT]),
                    scoreboard: Scoreboard::new([width, SCOREBOARD_HEIGHT], title),
                    fps_counter: FPSCounter::new(),
                }
            },
            #[cfg(not(feature = "display-fps"))]
            () => {
                Application {
                    assets: assets,
                    window: window,
                    field: Field::new([width, height - SCOREBOARD_HEIGHT]),
                    scoreboard: Scoreboard::new([width, SCOREBOARD_HEIGHT], title),
                }
            },
        };
        Ok(application)
    }

    /// Handle button events.
    fn on_button_change(&mut self, button_arguments: ButtonArgs) {
        match button_arguments.state {
            ButtonState::Press => self.on_button_pressed(button_arguments.button),
            ButtonState::Release => self.on_button_released(button_arguments.button),
        }
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
    fn on_render(&mut self, event: &Event, _render_arguments: &RenderArgs) {
        let font: PathBuf = self.assets.join("Anonymous Pro.ttf");
        let factory = self.window.factory.clone();
        let texture_settings = TextureSettings::new();
        let mut font = Glyphs::new(font, factory, texture_settings).unwrap();

        let field: &Field = &self.field;
        let scoreboard: &Scoreboard = &self.scoreboard;
        #[cfg(feature = "display-fps")]
        let fps: &str = &self.fps_counter.tick().to_string();

        let _ = self.window.draw_2d(event, |context, gl_graphics| {
            clear(color::BLACK, gl_graphics);

            field.on_render(context.trans(0.0, f64::from(SCOREBOARD_HEIGHT)), gl_graphics);
            scoreboard.on_render(&mut font, context.trans(0.0, 0.0), gl_graphics);

            #[cfg(feature = "display-fps")]
            {
                let size: u32 = 25;
                let margin: f64 = 10.0;
                let transformation = context.transform.trans(margin, (size as f64) + margin);
                let text_object = Text::new_color(color::GREEN, size);
                text_object.draw(fps, &mut font, &context.draw_state, transformation, gl_graphics);
            }
        });
    }

    /// Resize the application.
    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.field.on_resize(new_width, new_height - SCOREBOARD_HEIGHT);
        self.scoreboard.on_resize(new_width, SCOREBOARD_HEIGHT);
    }

    /// Update the application state.
    fn on_update(&mut self, update_arguments: &UpdateArgs) {
        self.field.on_update(update_arguments);
        self.scoreboard.on_update(self.field.get_player_scores());
    }

    /// Run the application.
    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            match event {
                Event::Input(input_event) => {
                    match input_event {
                        Input::Button(button_arguments) => self.on_button_change(button_arguments),
                        Input::Resize(width, height) => self.on_resize(width, height),
                        _ => {},
                    }
                },
                Event::Loop(loop_event) => {
                    match loop_event {
                        Loop::Render(render_arguments) => self.on_render(&event, &render_arguments),
                        Loop::Update(update_arguments) => self.on_update(&update_arguments),
                        _ => {},
                    }
                },
                _ => {},
            };
        }
    }
}
