mod grid;

use std::{io, process::Command};
use crate::grid::Grid;

fn main() {
    let grid_width: usize = 8;
    let grid_height: usize = 8;
    let mine_count: usize = 5;

    let mut grid = Grid::new(grid_width, grid_height, mine_count);
    
    // game loop
    loop {
        let mut input = String::new();
        
        Command::new("clear").status().unwrap();
        grid.print();

        // handle input
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        // check if selected tile is a mine
        if let Some((x, y)) = parse_coordinates(&input, grid_width, grid_height) {
            if grid.get_tile(x, y) == 1 {
                grid.set_tile(x, y, 5);
            }
        }
    }
}

// dont touch, works somehow
fn parse_coordinates(input: &str, width: usize, height: usize) -> Option<(usize, usize)> {
    let input = input.trim().to_uppercase();

    let mut chars = input.chars();

    let col_char = chars.next()?;
    if !col_char.is_ascii_alphabetic() {
        return None;
    }

    let row_str: String = chars.collect();
    let y = row_str.parse::<usize>().ok()?.checked_sub(1)?;

    let x = (col_char as u8).checked_sub(b'A')? as usize;

    if x >= width || y >= height {
        return None;
    }

    Some((x, y))
}