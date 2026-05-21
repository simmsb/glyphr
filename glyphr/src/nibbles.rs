#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
#[doc(alias = "Nibble")]
pub enum U4 {
    #[default]
    Dec00 = 0b0000_0000,
    Dec01 = 0b0000_0001,
    Dec02 = 0b0000_0010,
    Dec03 = 0b0000_0011,
    Dec04 = 0b0000_0100,
    Dec05 = 0b0000_0101,
    Dec06 = 0b0000_0110,
    Dec07 = 0b0000_0111,
    Dec08 = 0b0000_1000,
    Dec09 = 0b0000_1001,
    Dec10 = 0b0000_1010,
    Dec11 = 0b0000_1011,
    Dec12 = 0b0000_1100,
    Dec13 = 0b0000_1101,
    Dec14 = 0b0000_1110,
    Dec15 = 0b0000_1111,
}

impl U4 {
    /// Returns [`None`] if `byte` is out-of-bounds.
    ///
    /// ```
    /// # use u4::U4;
    /// U4::new(1).unwrap();
    /// assert!(U4::new(16).is_none());
    /// ```
    pub const fn new(byte: u8) -> Option<Self> {
        Some(match byte {
            0b0000_0000 => Self::Dec00,
            0b0000_0001 => Self::Dec01,
            0b0000_0010 => Self::Dec02,
            0b0000_0011 => Self::Dec03,
            0b0000_0100 => Self::Dec04,
            0b0000_0101 => Self::Dec05,
            0b0000_0110 => Self::Dec06,
            0b0000_0111 => Self::Dec07,
            0b0000_1000 => Self::Dec08,
            0b0000_1001 => Self::Dec09,
            0b0000_1010 => Self::Dec10,
            0b0000_1011 => Self::Dec11,
            0b0000_1100 => Self::Dec12,
            0b0000_1101 => Self::Dec13,
            0b0000_1110 => Self::Dec14,
            0b0000_1111 => Self::Dec15,
            _ => return None,
        })
    }

    pub const fn truncate(byte: u8) -> Self {
        unsafe { Self::new(byte & 0b0000_1111).unwrap_unchecked() }
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

#[repr(transparent)]
pub struct AsNibbles<'a>(pub &'a [u8]);

impl<'a> AsNibbles<'a> {
    pub fn len(&self) -> usize {
        self.0.len() * 2
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, ix: usize) -> Option<U4> {
        let byte = self.0.get(ix >> 1)?;
        Some(if ix & 1 == 0 {
            U4::truncate(*byte)
        } else {
            U4::truncate(*byte >> 4)
        })
    }

    pub unsafe fn get_unchecked(&self, ix: usize) -> U4 {
        let byte = unsafe { self.0.get_unchecked(ix >> 1) };
        if ix & 1 == 1 {
            U4::truncate(*byte)
        } else {
            U4::truncate(*byte >> 4)
        }
    }
}
