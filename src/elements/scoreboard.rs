// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Display information on the current game status.

use piston_window::Context;
use piston_window::G2d;
use piston_window::Glyphs;
use piston_window::Transformed;
use piston_window::character::CharacterCache;
use piston_window::text::Text;

use color;

/// Alignment of text.
enum TextAlignment {
    /// Align text on the left edge of the text's bounding box.
    Left,

    /// Align text in the center of the text's bounding box.
    Center,

    /// Align text on the right edge of the text's bounding box.
    Right,
}

impl TextAlignment {
    /// Compute the x-position where the text will be drawn depending on the alignment.
    pub fn align(&self, position_x: f64, width: f64) -> f64 {
        match *self {
            TextAlignment::Left => position_x,
            TextAlignment::Center => position_x - (width / 2.0),
            TextAlignment::Right => position_x - width,
        }
    }
}

/// The scoreboard displays information on the game, such as the current score and the name.
#[derive(Clone, Debug, Default)]
pub struct Scoreboard {
    /// The name of the game.
    title: String,

    /// The height of the scoreboard.
    height: u32,

    /// The width of the scoreboard.
    width: u32,

    /// The players' scores.
    scores: [isize; 2]
}

impl Scoreboard {
    /// Initialize a new scoreboard with a given `size` (`[width, height]`) and a `title`.
    pub fn new(size: [u32; 2], title: &str) -> Scoreboard {
        Scoreboard {
            title: String::from(title),
            height: size[1],
            width: size[0],
            scores: [0, 0]
        }
    }

    /// Determine the font size based on the height of the scoreboard.
    fn determine_font_size(&self) -> u32 {
        self.height / 2
    }

    /// Draw the given `text` aligned at `position_x` on the screen. The text is always vertically aligned at the middle
    /// of the scoreboard.
    fn draw_text(&self, text: &str, alignment: TextAlignment, position_x: f64, font: &mut Glyphs,
                 context: &Context, graphics: &mut G2d) {
        let size: u32 = self.determine_font_size();
        let width: f64 = font.width(size, text);

        // The vertical alignment is the middle of the scoreboard. The y-position is the baseline of the text.
        let y: f64 = ((self.height + size) as f64) / 2.0;
        let x: f64 = alignment.align(position_x, width);
        let transformation = context.transform.trans(x, y);

        let text_object = Text::new_color(color::WHITE, size);
        text_object.draw(text, font, &context.draw_state, transformation, graphics);
    }

    /// Render the scoreboard.
    pub fn on_render(&self, font: &mut Glyphs, context: Context, graphics: &mut G2d) {
        let center: f64 = (self.width as f64) / 2.0;
        let left_margin: f64 = 10.0;
        let right_margin: f64 = (self.width as f64) - left_margin;

        // Draw the title.
        self.draw_text(&self.title, TextAlignment::Center, center, font, &context, graphics);

        // Draw the left score.
        let score: &str = &self.scores[0].to_string();
        self.draw_text(score, TextAlignment::Left, left_margin, font, &context, graphics);

        // Draw the right score.
        let score: &str = &self.scores[1].to_string();
        self.draw_text(score, TextAlignment::Right, right_margin, font, &context, graphics);
    }

    /// Update the scoreboard.
    pub fn on_update(&mut self, scores: [isize; 2]) {
        self.scores = scores;
    }
}

#[cfg(test)]
mod tests {
    #![allow(trivial_casts)]

    use super::*;

    #[test]
    fn align_left() {
        let alignment = TextAlignment::Left;
        let x: f64 = alignment.align(50.0, 20.0);
        assert_eq!(x, 50.0);
    }

    #[test]
    fn align_center() {
        let alignment = TextAlignment::Center;
        let x: f64 = alignment.align(50.0, 20.0);
        assert_eq!(x, 40.0);
    }

    #[test]
    fn align_right() {
        let alignment = TextAlignment::Right;
        let x: f64 = alignment.align(50.0, 20.0);
        assert_eq!(x, 30.0);
    }

    #[test]
    fn new() {
        let scoreboard = Scoreboard::new([200, 100], "Mief");
        assert_eq!(scoreboard.title, String::from("Mief"));
        assert_eq!(scoreboard.width, 200);
        assert_eq!(scoreboard.height, 100);
    }

    quickcheck! {
        fn determine_font_size(height: u32) -> bool {
            let scoreboard = Scoreboard::new([100, height], "Mief");
            let font_size: u32 = scoreboard.determine_font_size();

            if height % 2 == 0 {
                font_size * 2 == height
            }
            else {
                font_size * 2 + 1 == height
            }
        }
    }

    #[test]
    fn on_update() {
        let mut scoreboard = Scoreboard::new([200, 100], "Mief");
        scoreboard.on_update([42, -42]);
        assert_eq!(scoreboard.scores, [42, -42]);
    }
}
