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
mod audio;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{find_player_start, load_maze, print_maze};
use crate::input::process_events;
use crate::renderer::{render_world_2d, render_world_3d, draw_sprite_billboard};
use crate::texture::TextureManager;
use crate::enemy::{Enemy, distance};
use crate::caster::is_blocked_by_wall;
use crate::audio::Audio;

use raylib::prelude::*;
use std::time::Instant;

enum GameState {
    Menu,
    Playing,
    Victory,
    GameOver,
}

/// Worker (T)
#[derive(Clone)]
struct Worker {
    pos: Vector2,
}
impl Worker {
    fn new(x: f32, y: f32) -> Self {
        Worker {
            pos: Vector2::new(x, y),
        }
    }
}

/// Chest (C)
#[derive(Clone)]
struct Chest {
    pos: Vector2,
    opened: bool, // nuevo: si ya fue abierto
}
impl Chest {
    fn new(x: f32, y: f32) -> Self {
        Chest {
            pos: Vector2::new(x, y),
            opened: false,
        }
    }
}

/// Busca todas las posiciones de un carácter específico en el maze
fn find_positions_in_maze(maze: &maze::Maze, target: char, block_size: usize) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == target {
                let x = (i * block_size) as f32 + (block_size as f32 / 2.0);
                let y = (j * block_size) as f32 + (block_size as f32 / 2.0);
                positions.push((x, y));
            }
        }
    }
    positions
}

fn main() {
    let level_files = vec!["maze.txt", "maze1.txt", "maze2.txt"];
    let mut current_level = 0usize;
    let block_size = 20usize;
    
    let mut maze = load_maze(level_files[current_level]);
    println!("Laberinto cargado: {}", level_files[current_level]);
    print_maze(&maze);
    
    let (start_x, start_y) = find_player_start(&maze)
        .expect("No se encontró posición inicial del jugador");
    let mut player = Player::new(start_x, start_y);
    
    let enemy_positions = find_positions_in_maze(&maze, 'F', block_size);
    let mut enemies: Vec<Enemy> = enemy_positions
        .iter()
        .map(|(x, y)| Enemy::new(*x, *y, 'F'))
        .collect();

    let worker_positions = find_positions_in_maze(&maze, 'T', block_size);
    let mut workers: Vec<Worker> = worker_positions
        .iter()
        .map(|(x, y)| Worker::new(*x, *y))
        .collect();

    let chest_positions = find_positions_in_maze(&maze, 'C', block_size);
    let mut chests: Vec<Chest> = chest_positions
        .iter()
        .map(|(x, y)| Chest::new(*x, *y))
        .collect();

    println!(
        "Enemigos: {}, Workers: {}, Cofres: {}",
        enemies.len(),
        workers.len(),
        chests.len()
    );
    
    let window_width = 1280;
    let window_height = 720;
    
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("OFF (The 3D version)")
        .build();
    
    rl.set_target_fps(60);

    let mut audio = Audio::new();
    let mut last_health = player.health;
    let texture_manager = TextureManager::new(&mut rl);
    let mut prev_mouse_x = rl.get_mouse_position().x;
    let mut state = GameState::Menu;
    let mut damage_overlay_alpha: f32 = 0.0;

    // === NUEVO: mensaje de cofre ===
    let mut chest_message_timer: Option<Instant> = None;

    while !rl.window_should_close() {
        match state {
            GameState::Menu => {
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
                    damage_overlay_alpha = 0.0;

                    enemies = find_positions_in_maze(&maze, 'F', block_size)
                        .iter()
                        .map(|(x, y)| Enemy::new(*x, *y, 'F'))
                        .collect();
                    workers = find_positions_in_maze(&maze, 'T', block_size)
                        .iter()
                        .map(|(x, y)| Worker::new(*x, *y))
                        .collect();
                    chests = find_positions_in_maze(&maze, 'C', block_size)
                        .iter()
                        .map(|(x, y)| Chest::new(*x, *y))
                        .collect();
                    
                    state = GameState::Playing;
                }
                if key_escape {
                    break;
                }
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("OFF - Presiona 1, 2, 3 para elegir la zona", 100, 100, 30, Color::WHITE);
                d.draw_text("Presiona Enter", 100, 140, 20, Color::WHITE);
                d.draw_text(&format!("Zona: {}", current_level + 1), 100, 180, 24, Color::YELLOW);
                d.draw_text("1 - zona 1 1, 2 - zona 2, 3 - zona 3", 100, 220, 20, Color::LIGHTGRAY);
                d.draw_text("ESC - exit", 100, 260, 20, Color::LIGHTGRAY);
            }

            GameState::Playing => {
                let mouse_pos = rl.get_mouse_position();
                let mouse_dx = mouse_pos.x - prev_mouse_x;
                prev_mouse_x = mouse_pos.x;

                let mut level_changed = false;
                if player.health > 0 {
                    level_changed = process_events(&rl, &mut player, &maze, block_size, mouse_dx);
                }

                if level_changed {
                    if current_level == level_files.len() - 1 {
                        state = GameState::Victory;
                        continue;
                    } else {
                        current_level += 1;
                        maze = load_maze(level_files[current_level]);
                        let (nx, ny) = find_player_start(&maze).expect("No start in next level");
                        player.pos.x = nx;
                        player.pos.y = ny;
                        player.health = 100;
                        damage_overlay_alpha = 0.0;

                        enemies = find_positions_in_maze(&maze, 'F', block_size)
                            .iter()
                            .map(|(x, y)| Enemy::new(*x, *y, 'F'))
                            .collect();
                        workers = find_positions_in_maze(&maze, 'T', block_size)
                            .iter()
                            .map(|(x, y)| Worker::new(*x, *y))
                            .collect();
                        chests = find_positions_in_maze(&maze, 'C', block_size)
                            .iter()
                            .map(|(x, y)| Chest::new(*x, *y))
                            .collect();
                    }
                }

                // === Enemigos ===
                for e in enemies.iter_mut() {
                    e.update(&player, &maze, block_size);
                    if distance(&e.pos, &player.pos) < 12.0 && player.health > 0 {
                        player.health = (player.health - 1).max(0);
                        audio.play_hit();
                    }
                }

                // === Cofres ===
                for c in chests.iter_mut() {
                    if !c.opened && distance(&c.pos, &player.pos) < 15.0 {
                        c.opened = true;
                        audio.play_chest();
                        chest_message_timer = Some(Instant::now());
                    }
                }

                // Actualiza overlay de daño
                if player.health < last_health {
                    damage_overlay_alpha = 0.6;
                }
                last_health = player.health;

                if player.health > 10 {
                    damage_overlay_alpha = (damage_overlay_alpha - 0.02).max(0.0);
                } else {
                    damage_overlay_alpha = 0.8;
                }

                if player.health <= 0 {
                    state = GameState::GameOver;
                    continue;
                }

                // === Render ===
                let mut fb = Framebuffer::new_buffer(window_width, window_height, Color::BLACK);
                render_world_3d(&mut fb, &maze, &player, block_size, &texture_manager);
                
                // Enemigos
                for e in enemies.iter() {
                    let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, e.pos.x, e.pos.y, &maze, block_size);
                    if !blocked {
                        draw_sprite_billboard(&mut fb, e.pos, &player, block_size, &texture_manager, "F");
                    }
                }

                // Workers
                for w in workers.iter() {
                    let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, w.pos.x, w.pos.y, &maze, block_size);
                    if !blocked {
                        draw_sprite_billboard(&mut fb, w.pos, &player, block_size, &texture_manager, "T");
                    }
                }

                // Cofres (solo se dibujan los no abiertos)
                for c in chests.iter() {
                    if !c.opened {
                        let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, c.pos.x, c.pos.y, &maze, block_size);
                        if !blocked {
                            draw_sprite_billboard(&mut fb, c.pos, &player, block_size, &texture_manager, "C");
                        }
                    }
                }

                let mut mini_fb = Framebuffer::new_buffer(240, 135, Color::BLACK);
                render_world_2d(&mut mini_fb, &maze, &player, block_size);
                
                let texture = rl.load_texture_from_image(&thread, &fb.buffer).unwrap();
                let mini_tex = rl.load_texture_from_image(&thread, &mini_fb.buffer).unwrap();
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_texture(&texture, 0, 0, Color::WHITE);
                d.draw_texture(&mini_tex, window_width - 250, 10, Color::WHITE);
                d.draw_text(&format!("HP: {}", player.health), 10, 10, 24, Color::RED);
                d.draw_text(&format!("Zona: {}", current_level + 1), 10, 40, 20, Color::YELLOW);

                // Mostrar mensaje "Joker Received" si el cofre fue abierto recientemente
                if let Some(start) = chest_message_timer {
                    if start.elapsed().as_secs_f32() < 2.0 {
                        let msg = "Joker recibido";
                        let text_width = d.measure_text(msg, 40);
                        d.draw_text(msg, (window_width - text_width) / 2, window_height / 2 - 30, 40, Color::YELLOW);
                    } else {
                        chest_message_timer = None;
                    }
                }

                if damage_overlay_alpha > 0.01 {
                    let color = Color::new(255, 0, 0, (damage_overlay_alpha * 255.0) as u8);
                    d.draw_rectangle_lines_ex(Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32), 25.0, color);
                }
            }

            GameState::Victory => {
                let key_menu = rl.is_key_pressed(KeyboardKey::KEY_M);
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("Bien hecho. Pero aún te falta purificar más zonas.", 300, 300, 40, Color::WHITE);
                d.draw_text("M para volver al menú", 300, 360, 24, Color::LIGHTGRAY);
                if key_menu {
                    state = GameState::Menu;
                }
            }

            GameState::GameOver => {
                let key_respawn = rl.is_key_pressed(KeyboardKey::KEY_R);
                let key_menu = rl.is_key_pressed(KeyboardKey::KEY_M);
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("Fallaste.", 500, 300, 60, Color::RED);
                d.draw_text("R para respawnear", 480, 380, 30, Color::LIGHTGRAY);
                d.draw_text("M para volver al menú", 450, 420, 24, Color::LIGHTGRAY);

                if key_respawn {
                    let (nx, ny) = find_player_start(&maze).unwrap();
                    player.pos.x = nx;
                    player.pos.y = ny;
                    player.health = 100;
                    damage_overlay_alpha = 0.0;
                    enemies = find_positions_in_maze(&maze, 'F', block_size)
                        .iter()
                        .map(|(x, y)| Enemy::new(*x, *y, 'F'))
                        .collect();
                    state = GameState::Playing;
                }
                if key_menu {
                    state = GameState::Menu;
                }
            }
        }
    }
}
