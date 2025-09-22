use raylib::math::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub a: f32, // ángulo de visión en radianes
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: std::f32::consts::PI / 3.0, // PI/3 como valor inicial
            fov: std::f32::consts::PI / 3.0,
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
        // Normalizar el ángulo entre 0 y 2π
        self.a = self.a % (2.0 * std::f32::consts::PI);
        if self.a < 0.0 {
            self.a += 2.0 * std::f32::consts::PI;
        }
    }

    pub fn try_move(&mut self, new_x: f32, new_y: f32, maze: &super::maze::Maze) -> bool {
        let block_size = 20.0; // Debe coincidir con el block_size usado en el render
        let i = (new_x / block_size) as usize;
        let j = (new_y / block_size) as usize;

        // Verificar si la nueva posición es válida (no es una pared)
        if j < maze.len() && i < maze[0].len() {
            let cell = maze[j][i];
            if cell != '#' && cell != 'E' && cell != 'L' {
                self.pos.x = new_x;
                self.pos.y = new_y;
                return true;
            }
        }
        false
    }
}
