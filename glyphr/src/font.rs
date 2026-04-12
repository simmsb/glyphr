//! # font.rs
//!
//! Contains structures used to describe generated fonts

use crate::GlyphrError;

/// Contains informations that are bound to the single glyph
pub struct Glyph<'a> {
    pub character: char,
    pub bitmap: &'a [u8],
    pub width: u8,
    pub height: u8,
    pub xmin: i8,
    pub ymin: i8,
    pub advance_width: i8,
}

/// Contains informations that are useful for every glyph
#[derive(Clone, Copy)]
pub struct Font<'a> {
    pub glyphs: &'a [Glyph<'a>],
    pub size: u8,
    pub ascent: i8,
    pub descent: i8,
}

impl<'a> Font<'a> {
    /// Returns a Result, Glyph if it's Ok, Err if the glyph is not found
    pub fn find_glyph(&self, ch: char) -> Result<&Glyph<'a>, GlyphrError> {
        self.glyphs
            .binary_search_by_key(&ch, |g| g.character)
            .map(|idx| &self.glyphs[idx])
            .map_err(|_| GlyphrError::InvalidGlyph(ch))
    }
}

/// Used to describe alignment on X axis
#[derive(Clone, Copy)]
pub enum AlignH {
    Left,
    Center,
    Right,
}

/// Used to describe alignment on Y axis
#[derive(Clone, Copy)]
pub enum AlignV {
    Top,
    Center,
    Baseline,
}
