use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::intersect::Intersect;

pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    ray_angle: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;

    loop {
        let cos = d * ray_angle.cos();
        let sin = d * ray_angle.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' ', 0.0);
        }

        if maze[j][i] == '#' || maze[j][i] == 'L' {
            // Calcular offset dentro del bloque
            let offset_x = x % block_size as f32;
            let offset_y = y % block_size as f32;

            // Determinar si el impacto fue vertical u horizontal
            let offset = if offset_x < offset_y {
                offset_x / block_size as f32
            } else {
                offset_y / block_size as f32
            };

            return Intersect::new(d, maze[j][i], offset);
        }

        d += 1.0;

        if d > 1000.0 {
            return Intersect::new(d, ' ', 0.0);
        }
    }
}

// Función para debug que dibuja el rayo
pub fn cast_ray_debug(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    ray_angle: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::new(255, 0, 0, 100)); // Rojo semitransparente

    loop {
        let cos = d * ray_angle.cos();
        let sin = d * ray_angle.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' ', 0.0);
        }

        if maze[j][i] == '#' || maze[j][i] == 'L' {
            framebuffer.set_pixel(x as i32, y as i32);

            // Calcular offset también para debug
            let offset_x = x % block_size as f32;
            let offset_y = y % block_size as f32;
            let offset = if offset_x < offset_y {
                offset_x / block_size as f32
            } else {
                offset_y / block_size as f32
            };

            return Intersect::new(d, maze[j][i], offset);
        }

        framebuffer.set_pixel(x as i32, y as i32);
        d += 1.0;

        if d > 1000.0 {
            return Intersect::new(d, ' ', 0.0);
        }
    }
}
