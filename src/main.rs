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
    // Nivel inicial
    let mut current_level = 0;
    let level_files = vec!["maze.txt", "maze1.txt", "maze2.txt"]; // Agregar más niveles aquí
    
    // Cargar laberinto inicial
    let mut maze = load_maze(level_files[current_level]);
    println!("Laberinto cargado: {}", level_files[current_level]);
    print_maze(&maze);
    
    // Encontrar posición inicial del jugador
    let (start_x, start_y) = find_player_start(&maze)
        .expect("No se encontró posición inicial del jugador (carácter 'P' o 'p')");
    
    let mut player = Player::new(start_x, start_y);
    
    // Dimensiones base para 2D
    let block_size = 20;
    
    // Dimensiones de ventana fija
    let window_width = 1920;
    let window_height = 1080;
    
    // Inicializar ventana en modo ventana (no fullscreen)
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("OFF (The 3D version)")
        .build();
    
    rl.set_target_fps(60);
    
    let texture_manager: TextureManager = TextureManager::new(&mut rl);
    
    let mut mode = "3D"; // Modo inicial
    
    while !rl.window_should_close() {
        // Cambiar modo con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        
        // Procesar eventos de input y verificar si hay cambio de nivel
        let level_changed = process_events(&rl, &mut player, &maze, block_size);
        
        // Si el jugador tocó una puerta de nivel
        if level_changed {
            current_level += 1;
            
            // Verificar si hay más niveles
            if current_level < level_files.len() {
                // Cargar siguiente nivel
                maze = load_maze(level_files[current_level]);
                println!("\n¡Nivel completado! Cargando: {}", level_files[current_level]);
                print_maze(&maze);
                
                // Encontrar nueva posición inicial
                let (new_x, new_y) = find_player_start(&maze)
                    .expect("No se encontró posición inicial en el nuevo nivel");
                
                player.pos.x = new_x;
                player.pos.y = new_y;
                player.a = std::f32::consts::PI / 3.0; // Resetear ángulo
            } else {
                // Volver al primer nivel (loop infinito)
                current_level = 0;
                maze = load_maze(level_files[current_level]);
                println!("\n¡Todos los niveles completados! Reiniciando...");
                print_maze(&maze);
                
                let (new_x, new_y) = find_player_start(&maze)
                    .expect("No se encontró posición inicial");
                
                player.pos.x = new_x;
                player.pos.y = new_y;
                player.a = std::f32::consts::PI / 3.0;
            }
        }
        
        // Calcular dimensiones del laberinto dinámicamente
        let maze_height_cells = maze.len();
        let maze_width_cells = maze.iter().map(|row| row.len()).max().unwrap_or(0);
        let maze_width = (maze_width_cells * block_size) as i32;
        let maze_height = (maze_height_cells * block_size) as i32;
        
        // Crear framebuffer según el modo
        let mut fb = if mode == "2D" {
            Framebuffer::new_buffer(
                maze_width,
                maze_height,
                Color::BLACK,
            )
        } else {
            Framebuffer::new_buffer(
                window_width,
                window_height,
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
        
        // Si es modo 2D, centrar el mapa en la ventana
        if mode == "2D" {
            let offset_x = (window_width - maze_width) / 2;
            let offset_y = (window_height - maze_height) / 2;
            d.draw_texture(&texture, offset_x, offset_y, Color::WHITE);
        } else {
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
        
        // Mostrar FPS y nivel actual
        d.draw_text(&format!("FPS: {}", d.get_fps()), 10, 10, 30, Color::GREEN);
        d.draw_text(&format!("Level: {}", current_level + 1), 10, 45, 30, Color::YELLOW);
    }
}
