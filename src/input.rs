use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) -> bool {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 0.05;
    
    let mut level_changed = false;
    
    // Rotación
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.rotate(-ROTATION_SPEED);
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.rotate(ROTATION_SPEED);
    }
    
    // Movimiento hacia adelante/atrás
    if window.is_key_down(KeyboardKey::KEY_UP) {
        level_changed = player.move_forward(MOVE_SPEED, maze);
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        level_changed = player.move_backward(MOVE_SPEED, maze) || level_changed;
    }
    
    // Movimiento lateral (strafe)
    if window.is_key_down(KeyboardKey::KEY_A) {
        let strafe_angle = player.a - std::f32::consts::FRAC_PI_2;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(new_x, new_y, maze) || level_changed;
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        let strafe_angle = player.a + std::f32::consts::FRAC_PI_2;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(new_x, new_y, maze) || level_changed;
    }
    
    level_changed
}
