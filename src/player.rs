use raylib::math::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
    pub health: i32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: std::f32::consts::PI / 3.0,
            fov: std::f32::consts::PI / 3.0,
            health: 100,
        }
    }

    pub fn move_forward(&mut self, distance: f32, maze: &super::maze::Maze) -> bool {
        let new_x = self.pos.x + distance * self.a.cos();
        let new_y = self.pos.y + distance * self.a.sin();
        self.try_move(new_x, new_y, maze)
    }

    pub fn move_backward(&mut self, distance: f32, maze: &super::maze::Maze) -> bool {
        let new_x = self.pos.x - distance * self.a.cos();
        let new_y = self.pos.y - distance * self.a.sin();
        self.try_move(new_x, new_y, maze)
    }

    pub fn rotate(&mut self, angle: f32) {
        self.a += angle;
        self.a = self.a % (2.0 * std::f32::consts::PI);
        if self.a < 0.0 {
            self.a += 2.0 * std::f32::consts::PI;
        }
    }

    /// Retorna true si tocó '$' o 'E' (según el nivel).
    pub fn try_move(&mut self, new_x: f32, new_y: f32, maze: &super::maze::Maze) -> bool {
        let steps = 6;
        let dx = (new_x - self.pos.x) / steps as f32;
        let dy = (new_y - self.pos.y) / steps as f32;
        let mut nx = self.pos.x;
        let mut ny = self.pos.y;

        for _ in 0..steps {
            nx += dx;
            ny += dy;

            let block_size = 20.0;
            let i = (nx / block_size) as isize;
            let j = (ny / block_size) as isize;

            if j < 0 || i < 0 {
                return false;
            }

            let j_usize = j as usize;
            let i_usize = i as usize;

            if j_usize >= maze.len() || i_usize >= maze[0].len() {
                return false;
            }

            let cell = maze[j_usize][i_usize];

            // Detectar puerta o salida
            if cell == '$' || cell == 'E' {
                self.pos.x = nx;
                self.pos.y = ny;
                return true;
            }

            if cell == '#' || cell == 'L' {
                return false;
            }
        }

        self.pos.x = new_x;
        self.pos.y = new_y;
        false
    }
}
