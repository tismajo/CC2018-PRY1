use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{Maze, render_maze, get_cell_color};
use crate::caster::cast_ray;
use crate::textures::TextureManager;
use raylib::prelude::*;

pub fn render_world_2d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    // Limpiar el framebuffer
    framebuffer.clear();
    
    // Renderizar el laberinto
    render_maze(framebuffer, maze, block_size);
    
    // Renderizar al jugador
    framebuffer.set_current_color(Color::YELLOW);
    let player_size = 8;
    framebuffer.draw_rect(
        (player.pos.x - player_size as f32 / 2.0) as i32,
        (player.pos.y - player_size as f32 / 2.0) as i32,
        player_size,
        player_size,
    );
    
    // Dibujar la dirección del jugador
    let direction_length = 20.0;
    let end_x = player.pos.x + direction_length * player.a.cos();
    let end_y = player.pos.y + direction_length * player.a.sin();
    
    framebuffer.set_current_color(Color::RED);
    // Línea simple para mostrar dirección
    let steps = direction_length as i32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let x = player.pos.x + t * (end_x - player.pos.x);
        let y = player.pos.y + t * (end_y - player.pos.y);
        framebuffer.set_pixel(x as i32, y as i32);
    }
}

pub fn render_world_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    texture_manager: &TextureManager,
) {
    // Limpiar el framebuffer
    framebuffer.clear();
    
    // Reducir resolución para mejor rendimiento (cada 2 píxeles)
    let ray_step = 2;
    let num_rays = framebuffer.width / ray_step;
    let half_height = framebuffer.height / 2;
    
    // Pre-calcular valores constantes
    let fov_half = player.fov / 2.0;
    let distance_to_projection_plane = (framebuffer.width as f32 / 2.0) / fov_half.tan();
    let inv_num_rays = player.fov / num_rays as f32;
    
    for i in 0..num_rays {
        // Calcular el ángulo del rayo (optimizado)
        let ray_angle = player.a - fov_half + (i as f32 * inv_num_rays);
        
        // Lanzar el rayo
        let intersect = cast_ray(maze, player, ray_angle, block_size);
        
        // Solo renderizar si hay una pared
        if intersect.impact == ' ' {
            // Dibujar solo el suelo y techo si no hay pared
            draw_floor_and_ceiling_fast(framebuffer, i * ray_step, 0, framebuffer.height, ray_step);
            continue;
        }
        
        // Calcular la altura de la pared (optimizado)
        let wall_height = (block_size as f32 / intersect.distance.max(0.1)) * distance_to_projection_plane;
        
        let wall_top = half_height - (wall_height * 0.5) as i32;
        let wall_bottom = half_height + (wall_height * 0.5) as i32;
        
        let wall_top_clamped = wall_top.max(0);
        let wall_bottom_clamped = wall_bottom.min(framebuffer.height);
        
        let x_pos = i * ray_step;
        
        // Dibujar el techo
        draw_floor_and_ceiling_fast(framebuffer, x_pos, 0, wall_top_clamped, ray_step);
        
        // Dibujar la pared con renderizado optimizado
        draw_wall_fast(
            framebuffer,
            texture_manager,
            intersect.impact,
            x_pos,
            wall_top_clamped,
            wall_bottom_clamped,
            intersect.distance,
            ray_step
        );
        
        // Dibujar el suelo
        draw_floor_and_ceiling_fast(framebuffer, x_pos, wall_bottom_clamped, framebuffer.height, ray_step);
    }
}

fn draw_floor_and_ceiling_fast(framebuffer: &mut Framebuffer, x: i32, y_start: i32, y_end: i32, width: i32) {
    let half_height = framebuffer.height / 2;
    
    // Pre-definir colores
    let ceiling_color = Color::new(30, 30, 60, 255);
    let floor_color = Color::new(50, 50, 50, 255);
    
    for y in y_start..y_end {
        let color = if y < half_height { ceiling_color } else { floor_color };
        framebuffer.set_current_color(color);
        
        // Dibujar múltiples píxeles horizontalmente para llenar el ancho
        for dx in 0..width {
            if x + dx < framebuffer.width {
                framebuffer.set_pixel(x + dx, y);
            }
        }
    }
}

fn draw_wall_fast(
    framebuffer: &mut Framebuffer,
    texture_manager: &TextureManager,
    wall_char: char,
    x: i32,
    wall_top: i32,
    wall_bottom: i32,
    distance: f32,
    width: i32,
) {
    let wall_height = wall_bottom - wall_top;
    
    if wall_height <= 0 {
        return;
    }
    
    // Calcular sombreado una sola vez
    let distance_factor = (distance / 300.0).min(1.0);
    let brightness = (1.0 - distance_factor * 0.6).max(0.2);
    
    // Verificar si hay textura
    let use_texture = texture_manager.has_texture(wall_char);
    
    if use_texture {
        // Renderizado con textura (simplificado)
        let texture_height = 64.0;
        let y_step = texture_height / wall_height as f32;
        
        for y in wall_top..wall_bottom {
            // Coordenadas de textura simplificadas
            let texture_y = ((y - wall_top) as f32 * y_step) as u32 % 64;
            let texture_x = (distance * 8.0) as u32 % 64; // Patrón más simple
            
            let mut color = texture_manager.get_pixel_color(wall_char, texture_x, texture_y);
            
            // Aplicar sombreado
            color.r = (color.r as f32 * brightness) as u8;
            color.g = (color.g as f32 * brightness) as u8;
            color.b = (color.b as f32 * brightness) as u8;
            
            framebuffer.set_current_color(color);
            
            // Dibujar múltiples píxeles horizontalmente
            for dx in 0..width {
                if x + dx < framebuffer.width {
                    framebuffer.set_pixel(x + dx, y);
                }
            }
        }
    } else {
        // Renderizado con color sólido (optimizado)
        let base_color = get_cell_color(wall_char);
        let color = Color::new(
            (base_color.r as f32 * brightness) as u8,
            (base_color.g as f32 * brightness) as u8,
            (base_color.b as f32 * brightness) as u8,
            base_color.a,
        );
        
        framebuffer.set_current_color(color);
        
        // Dibujar toda la columna de una vez
        for y in wall_top..wall_bottom {
            for dx in 0..width {
                if x + dx < framebuffer.width {
                    framebuffer.set_pixel(x + dx, y);
                }
            }
        }
    }
}
