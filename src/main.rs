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
    
    // Inicializar ventana con un tamaño decente (ajustable)
    let (mut rl, thread) = raylib::init()
        .size(maze_width, maze_height)
        .title("OFF (The 3D version)")
        .build();

    rl.set_target_fps(60);

    // IMPORTANTE: Cargar el TextureManager UNA SOLA VEZ fuera del loop
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

        // Procesar eventos de input
        process_events(&rl, &mut player, &maze, 20);

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

        // Renderizar según el modo actual (reutilizando el mismo texture_manager)
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
    }
}