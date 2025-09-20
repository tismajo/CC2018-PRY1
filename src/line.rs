use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        framebuffer.set_pixel(x0, y0);
        
        if x0 == x1 && y0 == y1 {
            break;
        }

        err2 = err;
        
        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
