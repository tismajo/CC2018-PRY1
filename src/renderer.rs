use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::{cast_ray, cast_ray_debug};
use crate::texture::TextureManager;
use raylib::math::Vector2;

pub fn render_world_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) {
    // Reducir carga de raycasting: procesar cada RAY_STEP píxeles horizontalmente
    let ray_step: usize = 3; // ajusta: más grande => menos raycasts => +fps (pero más pixelado)
    let num_rays = (framebuffer.width as usize + ray_step - 1) / ray_step;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Distancia al plano de proyección (valor razonable)
    let distance_to_projection_plane = (framebuffer.width as f32 / (2.0 * (player.fov / 2.0).tan())).abs();

    for i in 0..num_rays {
        let screen_x = (i * ray_step) as i32; // coordenada x real en framebuffer
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(maze, player, ray_angle, block_size);

        let safe_distance = intersect.distance.max(0.1);
        // Altura proyectada (tamaño del bloque / distancia multiplicado por distancia al plano)
        let stake_height = block_size as f32;
        let adjusted_height = (stake_height / safe_distance) * distance_to_projection_plane;

        let stake_top = (hh - (adjusted_height / 2.0)) as i32;
        let stake_bottom = (hh + (adjusted_height / 2.0)) as i32;

        let cell_char = intersect.impact;
        let texture_key = match cell_char {
            'E' => "OFF000",
            '#' => "OFF001",
            'L' => "OFF002",
            '$' => "OFF001",
            _ => "OFF000",
        };

        if let Some(image) = textures.get(texture_key) {
            let pixel_data = image.get_image_data();
            let width = image.width as usize;
            let height = image.height as usize;

            // Renderizar un column de ancho ray_step en x
            for y in stake_top..stake_bottom {
                if y >= 0 && y < framebuffer.height {
                    let texture_y = ((y - stake_top) as f32 / (stake_bottom - stake_top) as f32) * height as f32;
                    let texture_x = (intersect.offset * width as f32).min((width - 1) as f32);

                    let tx = texture_x as usize;
                    let ty = (texture_y as usize).min(height - 1);

                    let index = ty * width + tx;
                    if index < pixel_data.len() {
                        let pixel_color = pixel_data[index];
                        let color = if cell_char == '$' {
                            Color::new(10, 10, 10, 255)
                        } else {
                            let distance_factor = 1.0 / (safe_distance / 50.0 + 1.0);
                            Color::new(
                                (pixel_color.r as f32 * distance_factor) as u8,
                                (pixel_color.g as f32 * distance_factor) as u8,
                                (pixel_color.b as f32 * distance_factor) as u8,
                                255,
                            )
                        };

                        framebuffer.set_current_color(color);
                        // llenar la columna de ancho ray_step
                        for sx in 0..(ray_step as i32) {
                            let px = screen_x + sx;
                            if px >= 0 && px < framebuffer.width {
                                framebuffer.set_pixel(px, y);
                            }
                        }
                    }
                }
            }
        }

        // Suelo
        let floor_distance_factor = 1.0 / (safe_distance / 50.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            (50.0 * floor_distance_factor) as u8,
            (30.0 * floor_distance_factor) as u8,
            0,
            255,
        ));
        for y in stake_bottom..framebuffer.height {
            for sx in 0..(ray_step as i32) {
                let px = screen_x + sx;
                if px >= 0 && px < framebuffer.width {
                    framebuffer.set_pixel(px, y);
                }
            }
        }

        // Cielo
        let sky_distance_factor = 1.0 / (safe_distance / 60.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            0,
            0,
            (100.0 * sky_distance_factor) as u8,
            255,
        ));
        for y in 0..stake_top {
            for sx in 0..(ray_step as i32) {
                let px = screen_x + sx;
                if px >= 0 && px < framebuffer.width {
                    framebuffer.set_pixel(px, y);
                }
            }
        }
    }

    // --- Dibujar sprites tipo "billboard" (enemigos, cofres, workers) estilo DOOM ---
    // Para cada sprite: calcular ángulo relativo al jugador, proyectar su tamaño y dibujar si dentro del FOV.
    // Lista simple: buscar claves 'F','C','T' en textures.
    let sprite_keys = vec!["F", "C", "T"];
    for key in sprite_keys {
        if textures.get(key).is_none() {
            continue;
        }
    }

    // Nota: no tenemos la lista de enemigos aquí; asumo que la lista de enemigos la haces en main y
    // necesitas llamar a una función que dibuje sprites pasándole la lista. Sin embargo, para cambiar lo mínimo,
    // vamos a ofrecer una función pública separada `draw_sprite_billboard` abajo y dejar que main la use
    // si quieres dibujar sprites desde allí. (Alternativamente puedes pasar &Vec<Enemy> a esta función).
}

/// Dibuja un sprite (imagen) proyectado como billboard en el framebuffer.
/// - `sprite_pos` en coordenadas px del mundo (mismo sistema que player.pos)
/// - `player` con posición y ángulo
/// - `textures.get(name)` debe existir
pub fn draw_sprite_billboard(
    framebuffer: &mut Framebuffer,
    sprite_pos: Vector2,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
    key: &str,
) {
    if let Some(image) = textures.get(key) {
        let pixel_data = image.get_image_data();
        let tw = image.width as usize;
        let th = image.height as usize;

        // vector desde player hasta sprite
        let dx = sprite_pos.x - player.pos.x;
        let dy = sprite_pos.y - player.pos.y;
        let distance = (dx * dx + dy * dy).sqrt().max(0.001);

        // angulo del sprite
        let angle_to_sprite = dy.atan2(dx);
        // normalizar diferencia de angulo al rango -PI..PI
        let mut rel_angle = angle_to_sprite - player.a;
        while rel_angle > std::f32::consts::PI { rel_angle -= 2.0 * std::f32::consts::PI; }
        while rel_angle < -std::f32::consts::PI { rel_angle += 2.0 * std::f32::consts::PI; }

        // Si está fuera del FOV, no dibujar
        if rel_angle.abs() > player.fov / 2.0 + 0.3 { // margen pequeño
            return;
        }

        let framebuffer_w = framebuffer.width as f32;
        let framebuffer_h = framebuffer.height as f32;
        let distance_to_projection_plane = (framebuffer_w / (2.0 * (player.fov / 2.0).tan())).abs();

        // proyectar tamaño: asumimos sprite "alto" = block_size
        let sprite_height = (block_size as f32 / distance) * distance_to_projection_plane;
        let sprite_width = sprite_height * (tw as f32 / th as f32);

        let center_x = (0.5 + (rel_angle / player.fov)) * framebuffer_w;
        let top = framebuffer_h / 2.0 - sprite_height / 2.0;
        let left = center_x - sprite_width / 2.0;

        // sample texture -> dibujar rect píxel a píxel (nearest neighbor)
        for sy in 0..(sprite_height as i32) {
            let v = sy as f32 / sprite_height;
            let ty = ((v * th as f32).clamp(0.0, (th - 1) as f32)) as usize;
            let py = (top + sy as f32) as i32;
            if py < 0 || py >= framebuffer.height { continue; }

            for sx in 0..(sprite_width as i32) {
                let u = sx as f32 / sprite_width;
                let tx = ((u * tw as f32).clamp(0.0, (tw - 1) as f32)) as usize;
                let px = (left + sx as f32) as i32;
                if px < 0 || px >= framebuffer.width { continue; }

                let index = ty * tw + tx;
                if index >= pixel_data.len() { continue; }
                let pix = pixel_data[index];

                // usar alfa para transparencia (alpha > 10 -> dibujar)
                if pix.a > 10 {
                    // Oscurecer según distancia (simple)
                    let df = 1.0 / (distance / 50.0 + 1.0);
                    let color = Color::new(
                        (pix.r as f32 * df) as u8,
                        (pix.g as f32 * df) as u8,
                        (pix.b as f32 * df) as u8,
                        255,
                    );
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }
}

pub fn render_world_2d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    // Escalar laberinto para caber en framebuffer (minimap)
    // Tamaño del laberinto en pixeles
    let maze_w = maze[0].len() * block_size;
    let maze_h = maze.len() * block_size;
    let scale_x = framebuffer.width as f32 / maze_w as f32;
    let scale_y = framebuffer.height as f32 / maze_h as f32;
    let scale = scale_x.min(scale_y).max(0.0001);

    // dibujar celdas escaladas
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = (col_index * block_size) as f32 * scale;
            let yo = (row_index * block_size) as f32 * scale;
            let w = (block_size as f32 * scale).ceil() as i32;
            let h = (block_size as f32 * scale).ceil() as i32;

            // color según celda (reusar get_cell_color si quieres)
            let color = crate::maze::get_cell_color(cell);
            framebuffer.set_current_color(color);
            framebuffer.draw_rect(xo as i32, yo as i32, w, h);
        }
    }

    // dibujar player (centrado)
    framebuffer.set_current_color(Color::RED);
    let px = (player.pos.x * scale) as i32;
    let py = (player.pos.y * scale) as i32;
    framebuffer.draw_rect(px - 2, py - 2, 4, 4);

    // dibujar línea de dirección
    framebuffer.set_current_color(Color::YELLOW);
    let end_x = player.pos.x + 20.0 * player.a.cos();
    let end_y = player.pos.y + 20.0 * player.a.sin();
    crate::line::line(
        framebuffer,
        Vector2::new(player.pos.x * scale, player.pos.y * scale),
        Vector2::new(end_x * scale, end_y * scale),
    );
}
