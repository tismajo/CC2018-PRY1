use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn print_maze(maze: &Maze) {
    for row in maze {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn find_player_start(maze: &Maze, block_size: usize) -> Option<(f32, f32)> {
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'P' || cell == 'p' {
                // Devolver coordenadas en el centro de la celda usando el block_size proporcionado
                let x = (i as f32 * block_size as f32) + (block_size as f32 / 2.0);
                let y = (j as f32 * block_size as f32) + (block_size as f32 / 2.0);
                return Some((x, y));
            }
        }
    }
    None
}

// Función pública para obtener el color basado en el carácter
pub fn get_cell_color(cell: char) -> Color {
    match cell {
        '#' => Color::PURPLE,
        'M' => Color::LIGHTGRAY,
        'S' => Color::GREEN,
        'E' => Color::RED,
        'L' => Color::WHITESMOKE,
        'P' => Color::BLACK,
        'T' => Color::WHITE,
        'C' => Color::PURPLE,
        _ => Color::BLACK,
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    let color = get_cell_color(cell);
    framebuffer.set_current_color(color);
    framebuffer.draw_rect(
        xo as i32, 
        yo as i32, 
        block_size as i32, 
        block_size as i32
    );
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}