mod framebuffer;
mod line;
mod maze;
mod caster;
mod player;
mod input;
mod renderer;
mod intersect;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{find_player_start, load_maze, print_maze};
use crate::input::process_events;
use crate::renderer::{render_world_2d, render_world_3d};

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

    // Calcular dimensiones basadas en el laberinto
    let block_size = 20;
    let width = maze[0].len() * block_size;
    let height = maze.len() * block_size;
    
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("OFF (The 3D version)")
        .resizable()
        .build();

    rl.set_target_fps(60);

    let mut mode = "2D"; // Modo inicial
    let mut texture: Option<Texture2D> = None;

    while !rl.window_should_close() {
        // Obtener tamaño actual de ventana (resizable)
        let screen_width = rl.get_screen_width().max(1); // Ensure at least 1
        let screen_height = rl.get_screen_height().max(1); // Ensure at least 1

        // Calcular nuevo tamaño de bloque en función del maze y ventana
        // Add protection against division by zero
        let maze_width = maze[0].len().max(1);
        let maze_height = maze.len().max(1);
        
        let block_size_x = screen_width as usize / maze_width;
        let block_size_y = screen_height as usize / maze_height;
        let block_size = block_size_x.min(block_size_y).max(1); // evitar 0

        // Cambiar de modo con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Procesar input del jugador
        process_events(&rl, &mut player, &maze);

        // Framebuffer del tamaño de la ventana, fondo negro
        let mut fb = Framebuffer::new_buffer(screen_width, screen_height, Color::BLACK);
        
        // Renderizar según el modo actual
        if mode == "2D" {
            render_world_2d(&mut fb, &maze, &player, block_size);
        } else {
            render_world_3d(&mut fb, &maze, &player, block_size);
        }

        // SOLUTION 1: Proper error handling + SOLUTION 2: Texture management
        // Unload previous texture first to avoid resource exhaustion
        if let Some(old_texture) = texture.take() {
            // Texture will be automatically dropped when it goes out of scope
            // This frees the GPU resources
        }

        // Create new texture with proper error handling
        let texture_result = rl.load_texture_from_image(&thread, &fb.buffer);
        let new_texture = match texture_result {
            Ok(tex) => Some(tex),
            Err(e) => {
                eprintln!("Failed to load texture: {}", e);
                eprintln!("Framebuffer dimensions: {}x{}", fb.buffer.width(), fb.buffer.height());
                None
            }
        };

        texture = new_texture;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // Only draw if we have a valid texture
        if let Some(ref tex) = texture {
            d.draw_texture(tex, 0, 0, Color::WHITE);
        } else {
            // Draw error message if texture failed to load
            d.draw_text("Texture loading failed!", 10, 10, 20, Color::RED);
        }

        // Debug info
        d.draw_text(&format!("Pos: ({:.1}, {:.1})", player.pos.x, player.pos.y), 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Angle: {:.2} rad", player.a), 10, 30, 20, Color::WHITE);
        d.draw_text(&format!("FOV: {:.2} rad", player.fov), 10, 50, 20, Color::WHITE);
        d.draw_text(&format!("Mode: {} (Press M to toggle)", mode), 10, 70, 20, Color::WHITE);
        d.draw_text(&format!("Window: {}x{}", screen_width, screen_height), 10, 90, 20, Color::WHITE);
        d.draw_text(&format!("Block size: {}", block_size), 10, 110, 20, Color::WHITE);
        d.draw_text("Controls: Arrow keys to move, A/D to strafe", 10, screen_height - 30, 20, Color::WHITE);
        
        // Add texture info for debugging
        if let Some(ref tex) = texture {
            d.draw_text(&format!("Texture: {}x{}", tex.width, tex.height), 10, 130, 20, Color::GREEN);
        }
    }
    
    // Explicitly drop the texture before exiting to ensure clean shutdown
    if let Some(texture) = texture.take() {
        drop(texture);
    }
}