use std::fmt::Display;

use crate::error::MiniPngError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelType {
    BlackAndWhite, // 0 = black-and-white
    GrayLevels,    // 1 = gray levels
    Palette,       // 2 = palette
    RGB,           // 3 = 24 bits color
}

impl TryFrom<u8> for PixelType {
    type Error = MiniPngError;
    fn try_from(value: u8) -> Result<PixelType, Self::Error> {
        match value {
            0 => Ok(PixelType::BlackAndWhite),
            1 => Ok(PixelType::GrayLevels),
            2 => Ok(PixelType::Palette),
            3 => Ok(PixelType::RGB),
            _ => Err(MiniPngError::InvalidPixelType(value)),
        }
    }
}

impl Display for PixelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PixelType::BlackAndWhite => write!(f, "0 (1 bit black and white)")?,
            PixelType::GrayLevels => write!(f, "1 (8 bits gray levels)")?,
            PixelType::Palette => write!(f, "2 (8 bits palette)")?,
            PixelType::RGB => write!(f, "3 (24 bits rgb images)")?,
        }
        Ok(())
    }
}

impl PixelType {
    pub fn bit_size(&self) -> usize {
        match self {
            PixelType::BlackAndWhite => 1,
            PixelType::GrayLevels => 8,
            PixelType::Palette => 8,
            PixelType::RGB => 24,
        }
    }
}
