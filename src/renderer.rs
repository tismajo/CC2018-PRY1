use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::{cast_ray, cast_ray_debug};
use crate::intersect::Intersect;
use crate::maze::get_cell_color; // Importar la función de colores

pub fn render_world_3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let num_rays = framebuffer.width as usize;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;
    
    // Distancia al plano de proyección
    let distance_to_projection_plane = 277.0;
    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        
        // Lanzar rayo sin dibujar (para modo 3D)
        let intersect = cast_ray(maze, player, ray_angle, block_size);
        
        // Calcular altura del stake basada en la distancia
        let distance_to_wall = intersect.distance;
        
        // Evitar división por cero y distancias muy pequeñas
        let safe_distance = distance_to_wall.max(0.1);
        
        // Calcular altura del stake (proporcional al inverso de la distancia)
        let stake_height = (hh / safe_distance) * distance_to_projection_plane;
        
        // Calcular posición vertical del stake
        let stake_top = (hh - (stake_height / 2.0)) as i32;
        let stake_bottom = (hh + (stake_height / 2.0)) as i32;
        
        // Obtener el color base según el tipo de celda
        let base_color = get_cell_color(intersect.impact);
        
        // Ajustar la intensidad del color basado en la distancia
        let distance_factor = 1.0 / (safe_distance / 20.0 + 1.0);
        let color = Color::new(
            (base_color.r as f32 * distance_factor) as u8,
            (base_color.g as f32 * distance_factor) as u8,
            (base_color.b as f32 * distance_factor) as u8,
            255
        );
        
        framebuffer.set_current_color(color);
        
        // Dibujar el stake (línea vertical)
        for y in stake_top..stake_bottom {
            if y >= 0 && y < framebuffer.height {
                framebuffer.set_pixel(i as i32, y);
            }
        }
        
        // Dibujar suelo (usando un color consistente con el tema)
        let floor_distance_factor = 1.0 / (safe_distance / 30.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            (50.0 * floor_distance_factor) as u8,
            (30.0 * floor_distance_factor) as u8,
            0,
            255
        ));
        for y in stake_bottom..framebuffer.height {
            framebuffer.set_pixel(i as i32, y);
        }
        
        // Dibujar cielo (usando un color consistente)
        let sky_distance_factor = 1.0 / (safe_distance / 40.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            0,
            0,
            (100.0 * sky_distance_factor) as u8,
            255
        ));
        for y in 0..stake_top {
            framebuffer.set_pixel(i as i32, y);
        }
    }
}

pub fn render_world_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    // Renderizar laberinto
    crate::maze::render_maze(framebuffer, maze, block_size);
    
    // Dibujar jugador
    framebuffer.set_current_color(Color::RED);
    framebuffer.draw_rect(player.pos.x as i32 - 2, player.pos.y as i32 - 2, 4, 4);
    
    // Lanzar múltiples rayos para mostrar FOV (con debug)
    let num_rays = 60;
    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray_debug(framebuffer, maze, player, ray_angle, block_size);
    }
    
    // Dibujar dirección actual del jugador
    framebuffer.set_current_color(Color::YELLOW);
    let end_x = player.pos.x + 20.0 * player.a.cos();
    let end_y = player.pos.y + 20.0 * player.a.sin();
    crate::line::line(
        framebuffer,
        raylib::math::Vector2::new(player.pos.x, player.pos.y),
        raylib::math::Vector2::new(end_x, end_y)
    );
}
