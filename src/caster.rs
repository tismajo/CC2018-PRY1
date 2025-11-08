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
            // Calcular offset dentro del bloque para texturizado correcto
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            
            let offset_x = (x - block_x) / block_size as f32;
            let offset_y = (y - block_y) / block_size as f32;
            
            // Determinar qué lado del bloque fue golpeado
            let offset = if offset_x.abs() < 0.05 || (1.0 - offset_x).abs() < 0.05 {
                // Golpeó un lado vertical, usar offset Y
                offset_y
            } else if offset_y.abs() < 0.05 || (1.0 - offset_y).abs() < 0.05 {
                // Golpeó un lado horizontal, usar offset X
                offset_x
            } else {
                // Usar el menor para determinar cuál lado está más cerca
                if offset_x.min(1.0 - offset_x) < offset_y.min(1.0 - offset_y) {
                    offset_y
                } else {
                    offset_x
                }
            };
            
            return Intersect::new(d, maze[j][i], offset);
        }
        
        d += 1.0;
        if d > 1000.0 {
            return Intersect::new(d, ' ', 0.0);
        }
    }
}

pub fn cast_ray_debug(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    ray_angle: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::new(255, 0, 0, 100));
    
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
            
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            
            let offset_x = (x - block_x) / block_size as f32;
            let offset_y = (y - block_y) / block_size as f32;
            
            let offset = if offset_x.abs() < 0.05 || (1.0 - offset_x).abs() < 0.05 {
                offset_y
            } else if offset_y.abs() < 0.05 || (1.0 - offset_y).abs() < 0.05 {
                offset_x
            } else {
                if offset_x.min(1.0 - offset_x) < offset_y.min(1.0 - offset_y) {
                    offset_y
                } else {
                    offset_x
                }
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
