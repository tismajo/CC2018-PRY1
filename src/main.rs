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

    while !rl.window_should_close() {
        // Cambiar modo con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Procesar eventos de input
        process_events(&rl, &mut player, &maze, 20);

        // Crear nuevo framebuffer en cada frame
        let mut fb = Framebuffer::new_buffer(width as i32, height as i32, Color::WHITE);
        
        // Renderizar según el modo actual
        if mode == "2D" {
            render_world_2d(&mut fb, &maze, &player, block_size);
        } else {
            render_world_3d(&mut fb, &maze, &player, block_size);
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
        d.draw_text("Controls: Arrow keys to move, A/D to strafe", 10, height as i32 - 30, 20, Color::WHITE);
    }
}
