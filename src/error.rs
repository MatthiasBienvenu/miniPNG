use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniPngError {
    #[error("File is too small to be a Mini-PNG image")]
    FileTooSmall,

    #[error("File doesn't have the magic number \"Mini-PNG\"")]
    InvalidMagicNumber,

    #[error("Could not read the length field of the current block")]
    InvalidBlockLength,

    #[error("Block length is invalid")]
    BlockLengthMismatch,

    #[error("Could not read UTF-8 text from a Comment")]
    InvalidUtf8Comment,

    #[error("Found a second header block")]
    DuplicateHeader,

    #[error("Header block too small")]
    HeaderTooSmall,

    #[error("Found a second palette block")]
    DuplicatePalette,

    #[error("Invalid block type found: {0}")]
    InvalidBlockType(char),

    #[error("No header block found")]
    MissingHeader,

    #[error("No palette block found")]
    MissingPalette,

    #[error("No data block found")]
    MissingData,

    #[error("A palette block was found but pixel type is not palette")]
    UnexpectedPalette,

    #[error("Invalid palette index: {0}")]
    InvalidPaletteIndex(u8),

    #[error("Expected {expected} bits ({width}x{height} pixels) but found {found} bits in data")]
    DataSizeMismatch {
        expected: usize,
        found: usize,
        width: u32,
        height: u32,
    },

    #[error("Invalid pixel type: {0}")]
    InvalidPixelType(u8),

    #[error("Illegal character found: {0}")]
    IllegalCharacter(char),

    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: String,
        #[source]
        source: io::Error,
    },
}

pub type Result<T> = std::result::Result<T, MiniPngError>;
