mod framebuffer;
mod line;
mod maze;
mod caster;
mod player;
mod input;
mod renderer;
mod intersect;
mod textures;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{find_player_start, load_maze, print_maze};
use crate::input::process_events;
use crate::renderer::{render_world_2d, render_world_3d};
use crate::textures::TextureManager;
use raylib::prelude::*;

fn main() {
    // Configuración de resolución global
    const WINDOW_WIDTH: i32 = 500;
    const WINDOW_HEIGHT: i32 = 500;
    const BLOCK_SIZE: usize = 10;  // Reducido de 20 a 10 para que quepa mejor
    
    // Cargar laberinto desde archivo primero
    let maze = load_maze("maze.txt");
    println!("Laberinto cargado:");
    print_maze(&maze);
    
    // Encontrar posición inicial del jugador (ajustando al nuevo block_size)
    let (start_x, start_y) = find_player_start(&maze, BLOCK_SIZE)
        .expect("No se encontró posición inicial del jugador (carácter 'P' o 'p')");
    
    let mut player = Player::new(start_x, start_y);
    
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("OFF (The 3D version)")
        .build();
    
    rl.set_target_fps(60);
    
    // Cargar el TextureManager UNA SOLA VEZ fuera del loop
    let texture_manager = TextureManager::new(&mut rl, &thread);
    println!("TextureManager inicializado con {} texturas", 
                if texture_manager.is_initialized() { "éxito" } else { "fallback" });
    
    let mut mode = "2D"; // Modo inicial
    
    while !rl.window_should_close() {
        // Cambiar modo con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
            println!("Cambiando a modo: {}", mode);
        }
        
        // Procesar eventos de input (pasando el block_size correcto)
        process_events(&rl, &mut player, &maze, BLOCK_SIZE);
        
        // Crear framebuffer con resolución fija
        let mut fb = Framebuffer::new_buffer(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            Color::BLACK,
        );
        
        // Renderizar según el modo actual
        if mode == "2D" {
            render_world_2d(&mut fb, &maze, &player, BLOCK_SIZE);
        } else {
            render_world_3d(&mut fb, &maze, &player, BLOCK_SIZE, &texture_manager);
        }
        
        // Convertir framebuffer a textura
        let texture = rl.load_texture_from_image(&thread, &fb.buffer).unwrap();
        let fps = rl.get_fps();
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_text(&format!("Pos: ({:.1}, {:.1})", player.pos.x, player.pos.y), 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Angle: {:.2} rad", player.a), 10, 30, 20, Color::WHITE);
        d.draw_text(&format!("FOV: {:.2} rad", player.fov), 10, 50, 20, Color::WHITE);
        d.draw_text(&format!("Mode: {} (Press M to toggle)", mode), 10, 70, 20, Color::WHITE);
        d.draw_text(&format!("FPS: {}", fps), 10, 90, 20, Color::WHITE);
    }
}
