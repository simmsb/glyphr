//! # Glyphr
//!
//! This library focus is not to be the fastest, but one of the most beautiful in the embedded world.

#![no_std]

mod api;
mod font;
// mod renderer;
mod utils;

pub use api::{
    Glyphr, GlyphrError, RenderConfig, RenderTarget, TextAlign,
};
pub use font::{AlignH, AlignV, Font, Glyph};
pub use glyphr_macros::generate_font;

#[cfg(feature = "toml")]
pub use glyphr_macros::generate_fonts_from_toml;
