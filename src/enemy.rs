use raylib::math::Vector2;
use crate::player::Player;
use crate::maze::Maze;

/// Enemigo básico
#[derive(Clone)]
pub struct Enemy {
    pub pos: Vector2,
    pub texture_key: char,
    pub anim_offset: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Enemy {
            pos: Vector2::new(x, y),
            texture_key,
            anim_offset: 0.0,
        }
    }

    /// Mueve al enemigo hacia el jugador, evitando paredes de forma simple.
    pub fn update_towards_player(&mut self, player: &Player, _maze: &Maze) {
        let speed = 0.8;
        let dir_x = player.pos.x - self.pos.x;
        let dir_y = player.pos.y - self.pos.y;
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.001);
        let nx = self.pos.x + (dir_x / len) * speed;
        let ny = self.pos.y + (dir_y / len) * speed;

        // update
        self.pos.x = nx;
        self.pos.y = ny;

        // anim offset simple (oscila para simular "animación")
        self.anim_offset += 0.12;
        if self.anim_offset > std::f32::consts::PI * 2.0 {
            self.anim_offset -= std::f32::consts::PI * 2.0;
        }
    }
}

pub fn distance(v1: &Vector2, v2: &Vector2) -> f32 {
    ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt()
}