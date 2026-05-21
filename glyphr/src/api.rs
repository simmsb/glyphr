//! # rendered.rs
//!
//! This module describes the public API to this library.
//! Everything is done via the `Glyphr` struct.

use crate::{
    font::{AlignH, AlignV, Font},
    nibbles::{AsNibbles, U4},
};

/// Trait used to make a target writable by Glyphr.
pub trait RenderTarget {
    /// x and y are coordinates, while color contains an ARGB8888 encoded value. You should handle
    /// alpha blending on your own.
    fn write_pixel(&mut self, x: u32, y: u32, color: u32) -> bool;

    /// This function return a touple of (width, height) of the target.
    fn dimensions(&self) -> (u32, u32);
}
/// Configuration for text rendering.
#[derive(Clone, Copy)]
pub struct RenderConfig {}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Text alignment options.
#[derive(Clone, Copy)]
pub struct TextAlign {
    pub horizontal: AlignH,
    pub vertical: AlignV,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self {
            horizontal: AlignH::Left,
            vertical: AlignV::Top,
        }
    }
}

/// Main renderer struct. With this you can render code.
pub struct Glyphr {
    render_config: RenderConfig,
}

impl Default for Glyphr {
    /// Create a new text renderer with default configuration.
    fn default() -> Self {
        Self::new()
    }
}

impl Glyphr {
    /// Create a new text renderer with default configuration.
    pub fn new() -> Self {
        Self {
            render_config: RenderConfig::default(),
        }
    }

    /// Create a new text renderer with custom configuration.
    pub fn with_config(render_config: RenderConfig) -> Self {
        Self { render_config }
    }

    /// Update the render configuration.
    pub fn set_config(&mut self, config: RenderConfig) {
        self.render_config = config;
    }

    /// Get the current render configuration.
    pub fn config(&self) -> &RenderConfig {
        &self.render_config
    }

    #[inline(always)]
    pub fn pixels<'a>(
        &self,
        c: char,
        font: Font<'a>,
    ) -> Result<impl Iterator<Item = U4> + use<'a>, GlyphrError> {
        let glyph = font.find_glyph(c)?;
        let width = glyph.width;
        let nibbles = AsNibbles(glyph.bitmap);

        let it = itertools::iproduct!(0..(glyph.height as u8), 0..(glyph.width as u8)).map(
            move |(y, x)| unsafe {
                nibbles.get_unchecked(x as usize + y as usize * width as usize)
            },
        );

        Ok(it)
    }

    #[inline(always)]
    pub fn pixels_scaled<'a>(
        &self,
        c: char,
        font: Font<'a>,
        scale: u8,
    ) -> Result<impl Iterator<Item = U4> + use<'a>, GlyphrError> {
        let glyph = font.find_glyph(c)?;
        let width = glyph.width;
        let nibbles = AsNibbles(glyph.bitmap);

        let it = itertools::iproduct!(
            0..(scale * glyph.height as u8),
            0..(scale * glyph.width as u8)
        )
        .map(move |(y, x)| unsafe {
            nibbles.get_unchecked((x / scale) as usize + (y / scale) as usize * width as usize)
        });

        Ok(it)
    }
}

#[derive(Debug, Clone)]
pub enum GlyphrError {
    OutOfBounds,
    InvalidGlyph(char),
    InvalidTarget,
}

impl core::fmt::Display for GlyphrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GlyphrError::OutOfBounds => write!(f, "Rendering position is out of bounds"),
            GlyphrError::InvalidGlyph(c) => write!(f, "Glyph not found: '{c}'"),
            GlyphrError::InvalidTarget => write!(f, "Invalid render target"),
        }
    }
}

impl core::error::Error for GlyphrError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_config_default_values() {
        let cfg = SdfConfig::default();
        assert_eq!(cfg.size, 16);
        assert_eq!(cfg.mid_value, 0.5);
        assert_eq!(cfg.smoothing, 0.1);
    }

    #[test]
    fn test_render_config_default_values() {
        let cfg = RenderConfig::default();
        assert_eq!(cfg.sdf.size, 16);
        assert_eq!(cfg.sdf.mid_value, 0.5);
        assert_eq!(cfg.sdf.smoothing, 0.1);
    }

    #[test]
    fn test_glyphr_new_initializes_correctly() {
        let glyphr = Glyphr::new();

        assert_eq!(glyphr.render_config.sdf.size, 16);
        assert_eq!(glyphr.render_config.sdf.mid_value, 0.5);
        assert_eq!(glyphr.render_config.sdf.smoothing, 0.1);
    }

    #[test]
    fn test_pixel_callback_writes_color() {
        let mut buffer = [0u32; 16];
        let mut target = BufferTarget::new(&mut buffer, 4, 4);
        target.write_pixel(2, 1, 0xff123456);

        let idx = 1 * 4 + 2;
        assert_eq!(buffer[idx], 0xff123456);
    }

    #[test]
    fn test_buffer_target_dimentsions() {
        let mut buffer = [0u32; 16];
        let target = BufferTarget::new(&mut buffer, 4, 4);

        assert_eq!(target.dimensions(), (4, 4));
    }
}
