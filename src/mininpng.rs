use std::{fmt::Debug, path::PathBuf};

use crate::error::{MiniPngError, Result};
use crate::pixel_type::PixelType;

#[derive(Debug, Clone)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub pixel_type: PixelType,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<[u8; 3]>,
}

#[derive(Debug)]
pub struct MiniPNG {
    pub header: Header,
    pub palette: Option<Palette>,
    pub data: Vec<u8>,
    pub comments: Vec<String>,
}

impl MiniPNG {
    fn display_black_and_white(&self) -> Result<String> {
        let mut output = String::new();
        let total_pixels = (self.header.width * self.header.height) as usize;
        for i in 0..total_pixels {
            let byte = self.data[i / 8];

            if i % self.header.width as usize == 0 {
                output.push('\n');
            }

            let pixel = (byte >> (7 - (i % 8))) & 1;

            if pixel == 0 {
                output.push(' '); // black
            } else {
                output.push('X'); // white
            }
        }

        Ok(output)
    }

    fn display_grey_levels(&self) -> Result<String> {
        let mut output = String::new();
        for (i, pixel) in self.data.iter().enumerate() {
            if i % self.header.width as usize == 0 {
                output.push('\n');
            }

            output.push_str(&format!(
                "\x1b[38;2;{:?};{:?};{:?}m██\x1b[0m",
                pixel, pixel, pixel
            ));
        }

        Ok(output)
    }

    fn display_rgb(&self) -> Result<String> {
        let mut output = String::new();
        for (i, pixel) in self.data.chunks(3).enumerate() {
            if i % self.header.width as usize == 0 {
                output.push('\n');
            }

            output.push_str(&format!(
                "\x1b[38;2;{:?};{:?};{:?}m██\x1b[0m",
                pixel[0], pixel[1], pixel[2]
            ));
        }

        Ok(output)
    }

    fn display_palette(&self) -> Result<String> {
        let mut output = String::new();
        let palette = self.palette.as_ref().ok_or(MiniPngError::MissingPalette)?;

        for (i, &index) in self.data.iter().enumerate() {
            if i % self.header.width as usize == 0 {
                output.push('\n');
            }

            let color = palette
                .colors
                .get(index as usize)
                .ok_or(MiniPngError::InvalidPaletteIndex(index))?;
            output.push_str(&format!(
                "\x1b[38;2;{:?};{:?};{:?}m██\x1b[0m",
                color[0], color[1], color[2]
            ));
        }

        Ok(output)
    }

    // this function creates a MiniPNG image from a string
    // this string should contain 'X' (white) and ' ' (black)
    // all other token than newline chars, 'X' and ' '
    // will return an Err
    pub fn bw_from_string(input: &str) -> Result<Self> {
        let height = input.lines().count() as u32;
        let width = input
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u32;

        let total_pixels = (width * height) as usize;
        let mut data = vec![0x0; (total_pixels + 7) / 8];
        let lines: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

        for i in 0..total_pixels {
            let c = lines[i / width as usize]
                .get(i % width as usize)
                .copied()
                .unwrap_or(b' ');

            let bit: u8 = match c {
                b' ' => 0, // black
                b'X' => 1, // white
                _ => return Err(MiniPngError::IllegalCharacter(c as char)),
            };

            data[i / 8] |= bit << (7 - (i % 8));
        }

        Ok(MiniPNG {
            header: Header {
                width,
                height,
                pixel_type: PixelType::BlackAndWhite,
            },
            palette: None,
            data,
            comments: Vec::new(),
        })
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"Mini-PNG");

        // header
        bytes.push(b'H');
        bytes.extend_from_slice(&9u32.to_be_bytes());
        bytes.extend_from_slice(&self.header.width.to_be_bytes());
        bytes.extend_from_slice(&self.header.height.to_be_bytes());
        bytes.push(self.header.pixel_type as u8);

        // palette
        if let Some(palette) = &self.palette {
            bytes.push(b'P');
            bytes.extend_from_slice(&(palette.colors.len() as u32).to_be_bytes());

            for color in &palette.colors {
                bytes.extend_from_slice(color);
            }
        }

        // comments
        for comment in &self.comments {
            bytes.push(b'C');
            bytes.extend_from_slice(&(comment.len() as u32).to_be_bytes());
            bytes.extend_from_slice(comment.as_bytes());
        }

        // data
        bytes.push(b'D');
        bytes.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&self.data);

        std::fs::write(&path, bytes).map_err(|e| MiniPngError::FileWrite {
            path: path.display().to_string(),
            source: e,
        })
    }
}

impl TryFrom<Vec<u8>> for MiniPNG {
    type Error = MiniPngError;

    // this is the main minipng parsing function
    // it takes a data Vec or u8 and turns it into
    // a MiniPNG according to the specs
    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(MiniPngError::FileTooSmall);
        }
        if &bytes[0..8] != b"Mini-PNG" {
            return Err(MiniPngError::InvalidMagicNumber);
        }
        //  skip the "Mini-PNG" magic flag
        let mut iter = bytes.iter().skip(8);

        let mut header: Option<Header> = None;
        let mut palette: Option<Palette> = None;
        let mut data: Vec<u8> = Vec::new();
        let mut comments: Vec<String> = Vec::new();

        while let Some(&block_type) = iter.next() {
            let length = u32::from_be_bytes([
                *iter.next().ok_or(MiniPngError::InvalidBlockLength)?,
                *iter.next().ok_or(MiniPngError::InvalidBlockLength)?,
                *iter.next().ok_or(MiniPngError::InvalidBlockLength)?,
                *iter.next().ok_or(MiniPngError::InvalidBlockLength)?,
            ]);
            let content: Vec<u8> = iter.by_ref().take(length as usize).copied().collect();

            if content.len() != length as usize {
                return Err(MiniPngError::BlockLengthMismatch);
            }

            match block_type {
                b'C' => {
                    // this allows for utf8 encoded comments which is not explicitly supported by
                    // the spec which refers to ascii encoded comments
                    let text =
                        String::from_utf8(content).map_err(|_| MiniPngError::InvalidUtf8Comment)?;
                    comments.push(text);
                }

                b'D' => {
                    data.extend(content);
                }

                b'H' => {
                    if header.is_some() {
                        return Err(MiniPngError::DuplicateHeader);
                    }
                    if content.len() < 9 {
                        return Err(MiniPngError::HeaderTooSmall);
                    }

                    header = Some(Header {
                        width: u32::from_be_bytes([content[0], content[1], content[2], content[3]]),
                        height: u32::from_be_bytes([
                            content[4], content[5], content[6], content[7],
                        ]),
                        pixel_type: content[8].try_into()?,
                    });
                }

                b'P' => {
                    if palette.is_some() {
                        return Err(MiniPngError::DuplicatePalette);
                    }
                    palette = Some(Palette {
                        colors: content
                            .chunks_exact(3)
                            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                            .collect(),
                    });
                }

                _ => return Err(MiniPngError::InvalidBlockType(block_type as char)),
            }
        }

        let header = header.ok_or(MiniPngError::MissingHeader)?;

        if data.is_empty() {
            return Err(MiniPngError::MissingData);
        }

        match header.pixel_type {
            PixelType::Palette if palette.is_none() => {
                return Err(MiniPngError::MissingPalette);
            }
            _ if palette.is_some() && header.pixel_type != PixelType::Palette => {
                return Err(MiniPngError::UnexpectedPalette);
            }
            _ => {}
        }

        let bit_count = data.len() * 8;

        #[rustfmt::skip]
        let expected_bit_count = (
            (
                header.width as usize
                * header.height as usize
                * header.pixel_type.bit_size()
                + 7
            ) / 8
        ) * 8;

        if expected_bit_count != bit_count {
            return Err(MiniPngError::DataSizeMismatch {
                expected: expected_bit_count,
                found: bit_count,
                width: header.width,
                height: header.height,
            });
        }

        Ok(MiniPNG {
            header,
            palette,
            data,
            comments,
        })
    }
}

impl MiniPNG {
    pub fn display(&self) -> Result<String> {
        let mut output = String::new();
        output.push_str("Mini-PNG Image\n");
        output.push_str(&format!("Width: {}\n", self.header.width));
        output.push_str(&format!("Height: {}\n", self.header.height));
        output.push_str(&format!("Pixel Type: {}\n", self.header.pixel_type));
        output.push_str(&format!("Data size: {} bytes\n", self.data.len()));

        if let Some(palette) = &self.palette {
            output.push_str(&format!("Palette: {} colors\n", palette.colors.len()));
        }

        if !self.comments.is_empty() {
            output.push_str("Comments:\n");
            for comment in &self.comments {
                output.push_str(&format!("  - {}\n", comment));
            }
        }

        let image_data = match self.header.pixel_type {
            PixelType::BlackAndWhite => self.display_black_and_white()?,
            PixelType::GrayLevels => self.display_grey_levels()?,
            PixelType::RGB => self.display_rgb()?,
            PixelType::Palette => self.display_palette()?,
        };

        output.push_str(&image_data);
        Ok(output)
    }
}
