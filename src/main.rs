mod bmp;
mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::draw_line;

fn main() {
    let mut fb = Framebuffer::new(800, 600);

    // Fondo blanco
    fb.clear(255, 255, 255);

    // Línea 1: pendiente suave hacia abajo
    draw_line(
        &mut fb,
        100,
        100,
        700,
        180,
        0,
        0,
        0,
    );

    // Línea 2: pendiente más pronunciada hacia abajo
    draw_line(
        &mut fb,
        100,
        250,
        700,
        400,
        0,
        0,
        0,
    );

    // Línea 3: pendiente hacia arriba
    draw_line(
        &mut fb,
        100,
        520,
        700,
        440,
        0,
        0,
        0,
    );

    bmp::save_bmp("out.bmp", &fb)
        .expect("No se pudo guardar la imagen BMP");

    println!("Imagen guardada como out.bmp");
}