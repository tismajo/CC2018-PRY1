use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::{cast_ray, cast_ray_debug};
use crate::intersect::Intersect;
use crate::texture::TextureManager;

pub fn render_world_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) {
    let num_rays = framebuffer.width as usize;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;
    let distance_to_projection_plane = 277.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(maze, player, ray_angle, block_size);
        let safe_distance = intersect.distance.max(0.1);
        let stake_height = (hh / safe_distance) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)) as i32;
        let stake_bottom = (hh + (stake_height / 2.0)) as i32;

        let cell_char = intersect.impact;
        let texture_key = match cell_char {
            'E' => "OFF000",
            '#' => "OFF001",
            'L' => "OFF002",
            '3' => "OFF003",
            '4' => "OFF004",
            '5' => "OFF005",
            _ => "OFF000",
        };

        if let Some(image) = textures.get(texture_key) {
            let pixel_data = image.get_image_data();
            let width = image.width as usize;
            let height = image.height as usize;

            for y in stake_top..stake_bottom {
                if y >= 0 && y < framebuffer.height {
                    let texture_y = ((y - stake_top) as f32 / (stake_bottom - stake_top) as f32)
                        * height as f32;
                    let texture_x = (intersect.offset * width as f32) % width as f32;

                    let tx = texture_x as usize;
                    let ty = texture_y as usize;

                    if tx < width && ty < height {
                        let index = ty * width + tx;
                        let pixel_color = pixel_data[index];

                        let distance_factor = 1.0 / (safe_distance / 20.0 + 1.0);
                        let color = Color::new(
                            (pixel_color.r as f32 * distance_factor) as u8,
                            (pixel_color.g as f32 * distance_factor) as u8,
                            (pixel_color.b as f32 * distance_factor) as u8,
                            255,
                        );

                        framebuffer.set_current_color(color);
                        framebuffer.set_pixel(i as i32, y);
                    }
                }
            }
        }

        // Suelo
        let floor_distance_factor = 1.0 / (safe_distance / 30.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            (50.0 * floor_distance_factor) as u8,
            (30.0 * floor_distance_factor) as u8,
            0,
            255,
        ));
        for y in stake_bottom..framebuffer.height {
            framebuffer.set_pixel(i as i32, y);
        }

        // Cielo
        let sky_distance_factor = 1.0 / (safe_distance / 40.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            0,
            0,
            (100.0 * sky_distance_factor) as u8,
            255,
        ));
        for y in 0..stake_top {
            framebuffer.set_pixel(i as i32, y);
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
