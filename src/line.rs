use crate::framebuffer::Framebuffer;

pub fn draw_line(
    fb: &mut Framebuffer,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    red: u8,
    green: u8,
    blue: u8,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let step_x = if x0 < x1 { 1 } else { -1 };
    let step_y = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        fb.set_pixel(x0, y0, red, green, blue);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let double_error = 2 * error;

        if double_error >= dy {
            error += dy;
            x0 += step_x;
        }

        if double_error <= dx {
            error += dx;
            y0 += step_y;
        }
    }
}