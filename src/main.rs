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
use crate::renderer::{render_world_2d, render_world_3d, draw_sprite_billboard};
use crate::texture::TextureManager;
use crate::enemy::{Enemy, distance};
use crate::caster::is_blocked_by_wall;
use raylib::prelude::*;

enum GameState {
    Menu,
    Playing,
    Victory,
    GameOver,
}

/// Nueva estructura para representar workers (T en el mapa)
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
    
    // Buscar enemigos (F)
    let enemy_positions = find_positions_in_maze(&maze, 'F', block_size);
    let mut enemies: Vec<Enemy> = enemy_positions
        .iter()
        .map(|(x, y)| Enemy::new(*x, *y, 'F'))
        .collect();
    
    // Buscar workers (T)
    let worker_positions = find_positions_in_maze(&maze, 'T', block_size);
    let mut workers: Vec<Worker> = worker_positions
        .iter()
        .map(|(x, y)| Worker::new(*x, *y))
        .collect();
    
    println!("Enemigos encontrados: {}", enemies.len());
    println!("Workers encontrados: {}", workers.len());
    
    let window_width = 1280;
    let window_height = 720;
    
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("OFF (The 3D version)")
        .build();
    
    rl.set_target_fps(60);
    
    let texture_manager = TextureManager::new(&mut rl);
    let mut prev_mouse_x = rl.get_mouse_position().x;
    let mut state = GameState::Menu;
    let mut damage_overlay_alpha: f32 = 0.0; // Opacidad del borde rojo
    let mut last_health = player.health;

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

                    let enemy_pos = find_positions_in_maze(&maze, 'F', block_size);
                    enemies = enemy_pos.iter().map(|(x, y)| Enemy::new(*x, *y, 'F')).collect();
                    
                    let worker_pos = find_positions_in_maze(&maze, 'T', block_size);
                    workers = worker_pos.iter().map(|(x, y)| Worker::new(*x, *y)).collect();
                    
                    state = GameState::Playing;
                }
                
                if key_escape {
                    break;
                }
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("WELCOME - PRESS 1/2/3 TO SELECT A LEVEL", 100, 100, 30, Color::WHITE);
                d.draw_text("Press ENTER to start chosen level", 100, 140, 20, Color::WHITE);
                d.draw_text(&format!("Selected level: {}", current_level + 1), 100, 180, 24, Color::YELLOW);
                d.draw_text("1 - level 1, 2 - level 2, 3 - level 3", 100, 220, 20, Color::LIGHTGRAY);
                d.draw_text("ESC - quit", 100, 260, 20, Color::LIGHTGRAY);
            }

            GameState::Playing => {
                // Movimiento del mouse
                let mouse_pos = rl.get_mouse_position();
                let mouse_dx = mouse_pos.x - prev_mouse_x;
                prev_mouse_x = mouse_pos.x;

                // Procesar eventos solo si vivo
                let mut level_changed = false;
                if player.health > 0 {
                    level_changed = process_events(&rl, &mut player, &maze, block_size, mouse_dx);
                }

                // Si cambió de nivel
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

                        let enemy_pos = find_positions_in_maze(&maze, 'F', block_size);
                        enemies = enemy_pos.iter().map(|(x, y)| Enemy::new(*x, *y, 'F')).collect();
                        let worker_pos = find_positions_in_maze(&maze, 'T', block_size);
                        workers = worker_pos.iter().map(|(x, y)| Worker::new(*x, *y)).collect();
                    }
                }

                // Actualizar enemigos
                for e in enemies.iter_mut() {
                    e.update(&player, &maze, block_size);
                    if distance(&e.pos, &player.pos) < 12.0 && player.health > 0 {
                        player.health = (player.health - 1).max(0);
                    }
                }

                // Detectar daño reciente
                if player.health < last_health {
                    damage_overlay_alpha = 0.6;
                }
                last_health = player.health;

                // Reducir opacidad con el tiempo si > 10 HP
                if player.health > 10 {
                    damage_overlay_alpha = (damage_overlay_alpha - 0.02).max(0.0);
                } else {
                    damage_overlay_alpha = 0.8; // constante si salud baja
                }

                // Morir
                if player.health <= 0 {
                    state = GameState::GameOver;
                    continue;
                }

                // Renderizar mundo
                let mut fb = Framebuffer::new_buffer(window_width, window_height, Color::BLACK);
                render_world_3d(&mut fb, &maze, &player, block_size, &texture_manager);
                
                for e in enemies.iter() {
                    let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, e.pos.x, e.pos.y, &maze, block_size);
                    if !blocked {
                        draw_sprite_billboard(&mut fb, e.pos, &player, block_size, &texture_manager, "F");
                    }
                }

                for w in workers.iter() {
                    let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, w.pos.x, w.pos.y, &maze, block_size);
                    if !blocked {
                        draw_sprite_billboard(&mut fb, w.pos, &player, block_size, &texture_manager, "T");
                    }
                }

                // Minimap
                let mut mini_fb = Framebuffer::new_buffer(240, 135, Color::BLACK);
                render_world_2d(&mut mini_fb, &maze, &player, block_size);
                
                let texture = rl.load_texture_from_image(&thread, &fb.buffer).unwrap();
                let mini_tex = rl.load_texture_from_image(&thread, &mini_fb.buffer).unwrap();
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_texture(&texture, 0, 0, Color::WHITE);
                d.draw_texture(&mini_tex, window_width - 250, 10, Color::WHITE);
                d.draw_text(&format!("HP: {}", player.health), 10, 10, 24, Color::RED);
                d.draw_text(&format!("Level: {}", current_level + 1), 10, 40, 20, Color::YELLOW);

                // Borde rojo de daño
                if damage_overlay_alpha > 0.01 {
                    let color = Color::new(255, 0, 0, (damage_overlay_alpha * 255.0) as u8);
                    d.draw_rectangle_lines_ex(Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32), 25.0, color);
                }
            }

            GameState::Victory => {
                let key_menu = rl.is_key_pressed(KeyboardKey::KEY_M);
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("¡VICTORY! Has completado el juego.", 300, 300, 40, Color::WHITE);
                d.draw_text("Press M to return to menu", 300, 360, 24, Color::LIGHTGRAY);
                if key_menu {
                    state = GameState::Menu;
                }
            }

            GameState::GameOver => {
                let key_respawn = rl.is_key_pressed(KeyboardKey::KEY_R);
                let key_menu = rl.is_key_pressed(KeyboardKey::KEY_M);
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text("YOU DIED", 500, 300, 60, Color::RED);
                d.draw_text("Press R to Respawn", 480, 380, 30, Color::LIGHTGRAY);
                d.draw_text("Press M to return to Menu", 450, 420, 24, Color::LIGHTGRAY);

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
