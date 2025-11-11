use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;

pub fn process_events(
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
    mouse_dx: f32, // delta x del mouse esta frame
) -> bool {
    // movimiento relativo según frame; puedes ajustar
    const MOVE_SPEED: f32 = 4.0; // por frame (ajusta si quieres)
    const ROTATION_SPEED_KEY: f32 = 0.04;
    const MOUSE_SENSITIVITY: f32 = 0.0035;

    let mut level_changed = false;

    // Rotación por teclado
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.rotate(-ROTATION_SPEED_KEY);
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.rotate(ROTATION_SPEED_KEY);
    }

    // Rotación por mouse (solo horizontal delta)
    if mouse_dx.abs() > 0.0 {
        player.rotate(mouse_dx * MOUSE_SENSITIVITY);
    }

    // Adelante/atrás
    if window.is_key_down(KeyboardKey::KEY_UP) {
        level_changed = player.move_forward(MOVE_SPEED, maze);
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        level_changed = player.move_backward(MOVE_SPEED, maze) || level_changed;
    }

    // Strafe
    if window.is_key_down(KeyboardKey::KEY_A) {
        let strafe_angle = player.a - std::f32::consts::FRAC_PI_2;
        let nx = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let ny = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(nx, ny, maze) || level_changed;
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        let strafe_angle = player.a + std::f32::consts::FRAC_PI_2;
        let nx = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let ny = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(nx, ny, maze) || level_changed;
    }

    level_changed
}
