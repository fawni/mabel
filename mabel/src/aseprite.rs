// mabel, declarative pixel art
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::Path;

use mabel_aseprite::{
    cel::{CelContent, Pixels},
    AsepriteFile,
};

use crate::Result;

pub fn save_to_eno(ase_path: &str, output_path: &str) -> Result<()> {
    let ase = AsepriteFile::read_file(Path::new(ase_path))?;

    if ase.num_frames() != 1 {
        return Err(format!(
            "Expected 1 frame in aseprite file, found {} frames",
            ase.num_frames()
        )
        .into());
    }

    let mut image_pixels: Vec<Vec<[u8; 4]>> = vec![vec![[0, 0, 0, 0]; ase.width()]; ase.height()];

    for layer in ase.layers() {
        if !layer.is_visible() {
            continue;
        }

        let cel = ase.cel(0, layer.id());
        let raw_cel = cel.raw_cel().unwrap();
        let (starting_x, starting_y) = cel.top_left();

        match &raw_cel.content {
            CelContent::Raw(raw) => {
                let pixels = &raw.pixels;
                let cel_width = raw.size.width;
                let cel_height = raw.size.height;
                let (ending_x, ending_y) = (
                    starting_x + i32::from(cel_width),
                    starting_y + i32::from(cel_height),
                );
                match pixels {
                    Pixels::Rgba(pixels) => {
                        let mut pixels = pixels.iter();

                        for y in starting_y..ending_y {
                            for x in starting_x..ending_x {
                                let new_pixel = pixels.next().unwrap().0;
                                if let Some(old_pixel) =
                                    image_pixels[y as usize].get_mut(x as usize)
                                {
                                    if new_pixel[3] == 0 && old_pixel[3] != 0 {
                                        continue;
                                    }
                                    *old_pixel = new_pixel;
                                }
                            }
                        }
                    }
                    _ => return Err("Expected RGBA pixels".into()),
                }
            }
            _ => return Err("Expected raw cel content".into()),
        }
    }

    let mut palette: HashSet<[u8; 4]> = HashSet::new();

    for p in &image_pixels {
        for y in p {
            if y.eq(&[0, 0, 0, 0]) {
                continue;
            }

            palette.insert(*y);
        }
    }

    let mut chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut palette_map = HashMap::new();

    let mut eno = "size: 32\n".to_string();
    if !palette.is_empty() {
        eno.push_str("palette: \n");
        for color in palette {
            if color == [0, 0, 0, 0] {
                continue;
            }

            let char = chars.next().unwrap();
            palette_map.insert(color, char);
            eno.push_str(&format!(
                "{char} = rgba({}, {}, {}, {})\n",
                color[0],
                color[1],
                color[2],
                f32::from(color[3]) / 255.0
            ));
        }
    }

    palette_map.insert([0, 0, 0, 0], ' ');

    eno.push_str("\n-- pixels\n");
    for row in image_pixels {
        for cell in row {
            eno.push(*palette_map.get(&cell).unwrap());
        }

        eno.push('\n');
    }
    eno.push_str("-- pixels");

    let mut output_file = std::fs::File::create(output_path)?;
    output_file.write_all(eno.as_bytes())?;

    Ok(())
}
