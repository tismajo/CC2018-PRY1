use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::{cast_ray, cast_ray_debug};
use crate::texture::TextureManager;

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
            '3' => "OFF003",
            '4' => "OFF004",
            '5' => "OFF005",
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
}

pub fn render_world_2d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    crate::maze::render_maze(framebuffer, maze, block_size);

    framebuffer.set_current_color(Color::RED);
    framebuffer.draw_rect(player.pos.x as i32 - 2, player.pos.y as i32 - 2, 4, 4);

    let num_rays = 60;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray_debug(framebuffer, maze, player, ray_angle, block_size);
    }

    framebuffer.set_current_color(Color::YELLOW);
    let end_x = player.pos.x + 20.0 * player.a.cos();
    let end_y = player.pos.y + 20.0 * player.a.sin();
    crate::line::line(
        framebuffer,
        raylib::math::Vector2::new(player.pos.x, player.pos.y),
        raylib::math::Vector2::new(end_x, end_y),
    );
}
