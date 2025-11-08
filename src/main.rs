mod framebuffer;
mod line;
mod maze;
mod caster;
mod player;
mod input;
mod renderer;
mod intersect;
mod texture;
mod enemy;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{find_player_start, load_maze, print_maze};
use crate::input::process_events;
use crate::renderer::{render_world_2d, render_world_3d};
use crate::texture::TextureManager;
use raylib::prelude::*;

fn main() {
    // Cargar laberinto desde archivo primero
    let maze = load_maze("maze.txt");
    println!("Laberinto cargado:");
    print_maze(&maze);
    
    // Encontrar posición inicial del jugador
    let (start_x, start_y) = find_player_start(&maze)
        .expect("No se encontró posición inicial del jugador (carácter 'P' o 'p')");
    
    let mut player = Player::new(start_x, start_y);
    
    // Dimensiones base para 2D
    let block_size = 20;
    let maze_width = 1380;
    let maze_height = 940;
    
    // Dimensiones para modo 3D (ajustadas para 192px de ancho por columna)
    let window_width_3d = 1920;  // 10 columnas de 192px cada una
    let window_height_3d = 1080;
    
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(window_width_3d, window_height_3d)
        .title("OFF (The 3D version)")
        .build();
    
    rl.set_target_fps(60);
    
    let texture_manager: TextureManager = TextureManager::new(&mut rl);
    
    let mut mode = "3D"; // Modo inicial cambiado a 3D para ver el efecto
    
    while !rl.window_should_close() {
        // Cambiar modo con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
            
            // Ajustar tamaño de ventana según el modo
            if mode == "2D" {
                rl.set_window_size(maze_width, maze_height);
            } else {
                rl.set_window_size(window_width_3d, window_height_3d);
            }
        }
        
        // Procesar eventos de input
        process_events(&rl, &mut player, &maze, block_size);
        
        // Dimensiones de framebuffer según el modo
        let mut fb = if mode == "2D" {
            Framebuffer::new_buffer(
                maze_width as i32,
                maze_height as i32,
                Color::BLACK,
            )
        } else {
            Framebuffer::new_buffer(
                rl.get_screen_width(),
                rl.get_screen_height(),
                Color::BLACK,
            )
        };
        
        // Renderizar según el modo actual
        if mode == "2D" {
            render_world_2d(&mut fb, &maze, &player, block_size);
        } else {
            render_world_3d(&mut fb, &maze, &player, block_size, &texture_manager);        
        }
        
        // Convertir framebuffer a textura
        let texture = rl.load_texture_from_image(&thread, &fb.buffer).unwrap();
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        
        // Mostrar información de debug
        d.draw_text(&format!("Pos: ({:.1}, {:.1})", player.pos.x, player.pos.y), 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Angle: {:.2} rad", player.a), 10, 30, 20, Color::WHITE);
        d.draw_text(&format!("FOV: {:.2} rad", player.fov), 10, 50, 20, Color::WHITE);
        d.draw_text(&format!("Mode: {} (Press M to toggle)", mode), 10, 70, 20, Color::WHITE);
        d.draw_text("Arrow Keys: Move | A/D: Strafe | Left/Right: Rotate", 10, 90, 20, Color::WHITE);
    }
}
