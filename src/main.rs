// main.rs - Optimized version
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod enemy;

use textures::TextureManager;
use caster::cast_ray;
use player::{Player, process_events};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze, load_maze};
use enemy::Enemy;

use std::f32::consts::PI;

const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &mut Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    framebuffer.set_current_color(Color::YELLOW);

    let player_size = 10;
    for dx in 0..player_size {
        for dy in 0..player_size {
            framebuffer.set_pixel(
                player.pos.x as i32 + dx,
                player.pos.y as i32 + dy,
            );
        }
    }

    let num_rays = 5;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, ray_angle, block_size, true);
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &mut Player,
    texture_cache: &TextureManager,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    // Pre-calculate some values
    let fov_step = player.fov / num_rays as f32;
    let angle_start = player.a - (player.fov / 2.0);

    for i in 0..num_rays {
        let ray_angle = angle_start + (i as f32 * fov_step);
        let angle_diff = ray_angle - player.a;

        let intersect = cast_ray(framebuffer, &maze, &player, ray_angle, block_size, false);

        let d = intersect.distance;
        let impact = intersect.impact;
        
        // Corrected distance to avoid fisheye effect
        let corrected_distance = d * angle_diff.cos();
        let stake_height = (hh / corrected_distance) * 70.0;
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh - half_stake_height).max(0.0) as usize;
        let stake_bottom = (hh + half_stake_height).min(framebuffer.height as f32) as usize;

        // Skip if wall is too small to see
        if stake_bottom <= stake_top {
            continue;
        }

        // Pre-calculate texture sampling values
        let tx = intersect.tx;
        let wall_height = stake_bottom - stake_top;
        
        // Render wall column
        for y in stake_top..stake_bottom {
            let ty = ((y - stake_top) * 128 / wall_height) as u32;
            let color = texture_cache.get_pixel_color(impact, tx as u32, ty);

            framebuffer.set_pixel_with_color(i as i32, y as i32, color);
        }
    }
}

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    
    // Normalize angle difference
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    // Early culling: don't render if outside FOV
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // Distance culling
    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let end_x = (start_x + sprite_size as usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size as usize).min(framebuffer.height as usize);

    // Skip tiny sprites
    if end_x <= start_x || end_y <= start_y {
        return;
    }

    let sprite_width = end_x - start_x;
    let sprite_height = end_y - start_y;

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_width) as u32;
            let ty = ((y - start_y) * 128 / sprite_height) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color.a > 128 { // Simple alpha test instead of full comparison
                framebuffer.set_pixel_with_color(x as i32, y as i32, color);
            }
        }
    }
}

fn render_enemies(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemies: &[Enemy],
    texture_cache: &TextureManager,
) {
    for enemy in enemies {
        draw_sprite(framebuffer, player, enemy, texture_cache);
    }
}

fn main() {
    let window_width = 800; // Reduced from 1300 for better performance
    let window_height = 600; // Reduced from 900 for better performance
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("FNAF Doom - Optimized")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as u32, 
        window_height as u32, 
        Color::new(50, 50, 100, 255)
    );

    let maze = load_maze("maze.txt").expect("Failed to load maze");
    let mut player = Player { 
        pos: Vector2::new(150.0, 150.0),
        a: (PI / 2.0) as f32,
        fov: (PI / 3.0) as f32, // Slightly narrower FOV for performance
    }; 

    // Create enemies once at startup
    let enemies = vec![
        Enemy::new(250.0, 250.0, 'b'),
        Enemy::new(350.0, 300.0, 'f'),
        Enemy::new(350.0, 350.0, 'c'),
    ];

    let mut mode = "3D";
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);
    let target_fps = 60;
    window.set_target_fps(target_fps);

    let mut frame_count = 0;
    let mut fps_timer = std::time::Instant::now();

    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        
        framebuffer.clear();
        process_events(&window, &mut player, delta_time);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        
        if mode == "2D" {
           render_maze(&mut framebuffer, &maze, block_size, &mut player);
        } else {
            render_3d(&mut framebuffer, &maze, block_size, &mut player, &texture_cache);
            render_enemies(&mut framebuffer, &player, &enemies, &texture_cache);
        }    

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Simple FPS counter for debugging
        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            fps_timer = std::time::Instant::now();
        }
    }
}