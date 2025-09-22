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

        // Convertir coordenadas a posición en el laberinto
        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        // Verificar si estamos dentro de los límites del laberinto
        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' '); // Fuera de límites
        }

        // Si encontramos una pared, retornar la intersección
        if maze[j][i] == '#' || maze[j][i] == 'L' || maze[j][i] == 'E' {
            return Intersect::new(d, maze[j][i]);
        }

        d += 1.0;

        // Limitar la distancia máxima para evitar loops infinitos
        if d > 1000.0 {
            return Intersect::new(d, ' ');
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

        // Convertir coordenadas a posición en el laberinto
        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        // Verificar si estamos dentro de los límites del laberinto
        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' ');
        }

        // Si encontramos una pared, retornar la intersección
        if maze[j][i] == '#' || maze[j][i] == 'L' {
            framebuffer.set_pixel(x as i32, y as i32);
            return Intersect::new(d, maze[j][i]);
        }

        // Dibujar el rayo mientras no choque con pared
        framebuffer.set_pixel(x as i32, y as i32);
        d += 1.0;

        // Limitar la distancia máxima
        if d > 1000.0 {
            return Intersect::new(d, ' ');
        }
    }
}
