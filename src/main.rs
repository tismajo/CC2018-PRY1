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
use crate::enemy::{Enemy, distance};

use raylib::prelude::*;

enum GameState {
    Menu,
    Playing,
    Victory,
}

fn main() {
    // maze2.txt será el último nivel
    let level_files = vec!["maze.txt", "maze1.txt", "maze2.txt"];
    let mut current_level = 0usize;

    let mut maze = load_maze(level_files[current_level]);
    println!("Laberinto cargado: {}", level_files[current_level]);
    print_maze(&maze);

    let (start_x, start_y) = find_player_start(&maze)
        .expect("No se encontró posición inicial del jugador");

    let mut player = Player::new(start_x, start_y);
    let mut enemies: Vec<Enemy> = vec![Enemy::new(start_x + 200.0, start_y + 50.0, 'F')];

    let block_size = 20usize;
    let window_width = 1280;
    let window_height = 720;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("OFF (The 3D version) - Mejorado")
        .build();

    rl.set_target_fps(60);
    let texture_manager = TextureManager::new(&mut rl);

    let mut prev_mouse_x = rl.get_mouse_position().x;
    let mut state = GameState::Menu;

    while !rl.window_should_close() {
        // ===== MENÚ PRINCIPAL =====
        if let GameState::Menu = state {
            let key_1 = rl.is_key_pressed(KeyboardKey::KEY_ONE);
            let key_2 = rl.is_key_pressed(KeyboardKey::KEY_TWO);
            let key_3 = rl.is_key_pressed(KeyboardKey::KEY_THREE);
            let key_enter = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let key_escape = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);

            if key_1 { current_level = 0; }
            if key_2 && level_files.len() > 1 { current_level = 1; }
            if key_3 && level_files.len() > 2 { current_level = 2; }

            if key_enter {
                maze = load_maze(level_files[current_level]);
                let (sx, sy) = find_player_start(&maze).expect("No start found");
                player.pos.x = sx;
                player.pos.y = sy;
                player.a = std::f32::consts::PI / 3.0;
                player.health = 100;
                enemies = vec![Enemy::new(player.pos.x + 200.0, player.pos.y, 'F')];
                state = GameState::Playing;
            }

            if key_escape {
                break;
            }

            // ---- DIBUJO DEL MENÚ ----
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_text("WELCOME - PRESS 1/2/3 TO SELECT A LEVEL", 100, 100, 30, Color::WHITE);
            d.draw_text("Press ENTER to start chosen level", 100, 140, 20, Color::WHITE);
            d.draw_text(&format!("Selected level: {}", current_level + 1), 100, 180, 24, Color::YELLOW);
            d.draw_text("1 - level 1, 2 - level 2, 3 - level 3", 100, 220, 20, Color::LIGHTGRAY);
            d.draw_text("ESC - quit", 100, 260, 20, Color::LIGHTGRAY);
            continue;
        }

        // ===== GAMEPLAY =====
        let mouse_pos = rl.get_mouse_position();
        let mouse_dx = mouse_pos.x - prev_mouse_x;
        prev_mouse_x = mouse_pos.x;

        // Movimiento bloqueado si jugador muerto
        let mut level_changed = false;
        if player.health > 0 {
            level_changed = process_events(&rl, &mut player, &maze, block_size, mouse_dx);
        }

        // === Detectar si tocó una salida ===
        if level_changed {
            // Si está en el último nivel y tocó E, ganar
            if current_level == level_files.len() - 1 {
                // Solo termina si tocó la E de maze2.txt
                state = GameState::Victory;
            } else {
                // Avanzar nivel
                current_level += 1;
                if current_level < level_files.len() {
                    maze = load_maze(level_files[current_level]);
                    println!("\n¡Nivel completado! Cargando: {}", level_files[current_level]);
                    print_maze(&maze);
                    let (nx, ny) = find_player_start(&maze).expect("No start in next level");
                    player.pos.x = nx;
                    player.pos.y = ny;
                    player.a = std::f32::consts::PI / 3.0;
                }
            }
        }

        // === Actualizar enemigos ===
        for e in enemies.iter_mut() {
            e.update_towards_player(&player, &maze);
            if distance(&e.pos, &player.pos) < 12.0 {
                if player.health > 0 {
                    player.health = (player.health - 1).max(0);
                }
            }
        }

        // === Renderizado (3D + 2D) ===
        let mut fb = Framebuffer::new_buffer(window_width, window_height, Color::BLACK);
        render_world_3d(&mut fb, &maze, &player, block_size, &texture_manager);

        // Dibujar enemigos
        for e in enemies.iter() {
            let key = e.texture_key.to_string();
            crate::renderer::draw_sprite_billboard(
                &mut fb,
                e.pos,
                &player,
                block_size,
                &texture_manager,
                &key,
            );
        }

        // Minimap
        let mut mini_fb = Framebuffer::new_buffer(240, 135, Color::BLACK);
        render_world_2d(&mut mini_fb, &maze, &player, block_size);
        mini_fb.set_current_color(Color::ORANGE);
        for e in enemies.iter() {
            let px = e.pos.x * (mini_fb.width as f32 / (maze[0].len() as f32 * block_size as f32));
            let py = e.pos.y * (mini_fb.height as f32 / (maze.len() as f32 * block_size as f32));
            mini_fb.draw_rect(px as i32 - 2, py as i32 - 2, 4, 4);
        }

        let texture = rl.load_texture_from_image(&thread, &fb.buffer).unwrap();
        let mini_tex = rl.load_texture_from_image(&thread, &mini_fb.buffer).unwrap();

        let key_respawn = rl.is_key_pressed(KeyboardKey::KEY_R);
        let key_menu = rl.is_key_pressed(KeyboardKey::KEY_M);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        // HUD
        d.draw_texture(&mini_tex, window_width - 250, 10, Color::WHITE);
        d.draw_text(&format!("HP: {}", player.health), 10, 10, 24, Color::RED);
        d.draw_text(&format!("Level: {}", current_level + 1), 10, 40, 20, Color::YELLOW);
        d.draw_text(&format!("FPS: {}", d.get_fps()), 10, 70, 20, Color::GREEN);

        if let GameState::Victory = state {
            d.draw_text("¡VICTORY! Has completado el juego.", 300, 300, 40, Color::WHITE);
            d.draw_text("Press M to return to menu", 300, 360, 24, Color::LIGHTGRAY);
        }

        if player.health <= 0 {
            d.draw_text("YOU DIED - Press R to respawn", 400, 400, 30, Color::RED);
        }
        drop(d);

        // === Lógica post-dibujo ===
        if let GameState::Victory = state {
            if key_menu {
                current_level = 0;
                state = GameState::Menu;
            }
        }

        if player.health <= 0 && key_respawn {
            let (nx, ny) = find_player_start(&maze).expect("No start to respawn");
            player.pos.x = nx;
            player.pos.y = ny;
            player.health = 100;
        }
    }
}
