use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 3.0;
    const ROTATION_SPEED: f32 = 0.05;

    // Rotación
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.rotate(-ROTATION_SPEED);
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.rotate(ROTATION_SPEED);
    }

    // Movimiento hacia adelante/atrás
    if window.is_key_down(KeyboardKey::KEY_UP) {
        player.move_forward(MOVE_SPEED, maze, block_size);
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        player.move_backward(MOVE_SPEED, maze, block_size);
    }

    // Movimiento lateral (strafe)
    if window.is_key_down(KeyboardKey::KEY_A) {
        let strafe_angle = player.a - std::f32::consts::FRAC_PI_2;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        player.try_move(new_x, new_y, maze, block_size);
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        let strafe_angle = player.a + std::f32::consts::FRAC_PI_2;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        player.try_move(new_x, new_y, maze, block_size);
    }
}
