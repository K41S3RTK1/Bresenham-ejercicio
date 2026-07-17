use std::fs::File;
use std::io::{self, Write};

use crate::framebuffer::Framebuffer;

pub fn save_bmp(
    filename: &str,
    framebuffer: &Framebuffer,
) -> io::Result<()> {
    let width = framebuffer.width;
    let height = framebuffer.height;

    let row_size = width * 3;
    let padding = (4 - row_size % 4) % 4;
    let padded_row_size = row_size + padding;

    let pixel_data_size = padded_row_size * height;
    let file_size = 14 + 40 + pixel_data_size;

    let mut file = File::create(filename)?;

    // Encabezado del archivo BMP
    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&(54_u32).to_le_bytes())?;

    // Encabezado de información
    file.write_all(&(40_u32).to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&(1_u16).to_le_bytes())?;
    file.write_all(&(24_u16).to_le_bytes())?;
    file.write_all(&(0_u32).to_le_bytes())?;
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&(2835_i32).to_le_bytes())?;
    file.write_all(&(2835_i32).to_le_bytes())?;
    file.write_all(&(0_u32).to_le_bytes())?;
    file.write_all(&(0_u32).to_le_bytes())?;

    let padding_bytes = vec![0_u8; padding];

    // BMP guarda las filas desde abajo hacia arriba.
    for y in (0..height).rev() {
        for x in 0..width {
            let index = (y * width + x) * 3;

            let red = framebuffer.pixels[index];
            let green = framebuffer.pixels[index + 1];
            let blue = framebuffer.pixels[index + 2];

            // BMP utiliza el orden BGR.
            file.write_all(&[blue, green, red])?;
        }

        file.write_all(&padding_bytes)?;
    }

    Ok(())
}