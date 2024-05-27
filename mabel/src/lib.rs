// mabel, declarative pixel art
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use color_art::Color;
use png::Encoder;
use std::{collections::HashMap, vec};

#[cfg(feature = "aseprite")]
pub mod aseprite;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub type Palette = HashMap<char, Pixel>;

#[derive(Debug, Clone, Copy)]
pub enum Pixel {
    Trans,
    Colored(Color),
}

#[derive(Debug)]
pub struct Mabel {
    pub size: u8,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub palette: Palette,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Mabel {
    pub fn new(
        size: u8,
        width: Option<u32>,
        height: Option<u32>,
        palette: Palette,
        pixels: Vec<Vec<Pixel>>,
    ) -> Self {
        Self {
            size,
            width,
            height,
            palette,
            pixels,
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;

        Self::from(&mabel_eno::parse(&file)?)
    }

    pub fn from(eno: &mabel_eno::Document) -> Result<Self> {
        let size = eno
            .field("size")?
            .optional_value()?
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(1);

        let width = eno
            .field("width")?
            .optional_value()?
            .and_then(|s| s.parse::<u32>().ok());

        let height = eno
            .field("height")?
            .optional_value()?
            .and_then(|s| s.parse::<u32>().ok());

        let mut palette = eno
            .field("palette")?
            .attributes()?
            .iter()
            .map(|a| {
                if a.key().len() != 1 {
                    return Err(format!(
                        "Invalid palette key \"{}\", must be a single character",
                        a.key()
                    )
                    .into());
                };

                let k = a.key().chars().next().unwrap();
                let v = Pixel::Colored(a.required_value()?);

                Ok((k, v))
            })
            .collect::<Result<Palette>>()?;
        palette.insert(' ', Pixel::Trans);

        let mut pixels = vec![];
        for line in eno.embed("pixels")?.required_value::<String>()?.lines() {
            let mut line_pixels = vec![];
            for c in line.chars() {
                let p = palette
                    .get(&c)
                    .ok_or_else(|| format!("Unknown palette key \"{c}\""))?;
                line_pixels.push(*p);
            }
            pixels.push(line_pixels);
        }

        let mabel = Self::new(size, width, height, palette, pixels);

        if mabel.is_over_width() {
            return Err("Horizontal pixels are more than the specified width".into());
        }

        if mabel.is_over_height() {
            return Err("Vertical pixels are more than the specified height".into());
        }

        Ok(mabel)
    }

    pub fn save_png(&self, path: &str) -> Result<()> {
        let palette = self.palette();
        let file = std::fs::File::create(path)?;
        let w = std::io::BufWriter::new(file);

        let mut img = Encoder::new(w, self.image_width(), self.image_height());
        img.set_color(png::ColorType::Rgba);
        img.set_depth(png::BitDepth::Eight);

        let mut writer = img.write_header()?;
        writer.write_image_data(&palette)?;

        Ok(())
    }

    pub fn height(&self) -> u32 {
        self.height.unwrap_or_else(|| self.pixels_height())
    }

    pub fn width(&self) -> u32 {
        self.width.unwrap_or_else(|| self.pixels_width())
    }

    pub fn pixels_height(&self) -> u32 {
        self.pixels.len() as u32
    }

    pub fn pixels_width(&self) -> u32 {
        self.pixels.iter().map(Vec::len).max().unwrap_or(0) as u32
    }

    pub fn image_height(&self) -> u32 {
        self.height() * u32::from(self.size)
    }

    pub fn image_width(&self) -> u32 {
        self.width() * u32::from(self.size)
    }

    pub fn is_over_width(&self) -> bool {
        self.width
            .map_or(false, |width| self.pixels_width() > width)
    }

    pub fn is_over_height(&self) -> bool {
        self.height
            .map_or(false, |height| self.pixels_height() > height)
    }

    pub fn palette(&self) -> Vec<u8> {
        let mut palette = vec![];

        for line in &self.pixels {
            for _ in 0..self.size {
                if line.is_empty() {
                    for _ in 0..self.width() {
                        for _ in 0..u32::from(self.size) {
                            palette.push(0);
                            palette.push(0);
                            palette.push(0);
                            palette.push(0);
                        }
                    }
                    continue;
                }

                for pixel in line {
                    match pixel {
                        Pixel::Colored(c) => {
                            for _ in 0..u32::from(self.size) {
                                palette.push(c.red());
                                palette.push(c.green());
                                palette.push(c.blue());
                                palette.push((c.alpha() * 255.0) as u8);
                            }
                        }
                        Pixel::Trans => {
                            for _ in 0..u32::from(self.size) {
                                palette.push(0);
                                palette.push(0);
                                palette.push(0);
                                palette.push(0);
                            }
                        }
                    }
                }

                if line.len() != self.width() as usize {
                    for _ in 0..self.width() as usize - line.len() {
                        for _ in 0..u32::from(self.size) {
                            palette.push(0);
                            palette.push(0);
                            palette.push(0);
                            palette.push(0);
                        }
                    }
                }
            }
        }

        palette
    }
}
