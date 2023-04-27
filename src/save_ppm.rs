// Unused

use crate::{Color, IMAGE_HEIGHT, IMAGE_WIDTH};
use std::{fs::File, io::prelude::*};

pub fn save_ppm(filename: String, pixels: &[Color]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "255")?;
    for Color([r, g, b]) in pixels {
        let to255 = |x| (x * 255.99) as u8;
        writeln!(file, "{} {} {}", to255(r), to255(g), to255(b))?;
    }
    file.flush()?;
    Ok(())
}
