use rand::Rng;
use std::{io, process::Command};

fn main() {
    let grid_width: usize = 8;
    let grid_height: usize = 8;
    let mine_count: usize = 5;

    let mut grid = generate_grid(grid_width, grid_height, mine_count);
    
    // game loop
    loop {
        let mut input = String::new();
        
        Command::new("clear").status().unwrap();
        print_grid(&grid);

        // handle input
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        // check if selected tile is a mine
        if let Some((x, y)) = parse_coordinates(&input, grid_width, grid_height) {
            if grid[y][x] == 1 {
                grid = generate_grid(grid_width, grid_height, mine_count);
            }
        }
    }
}

fn generate_grid(width: usize, height: usize, mine_count: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; width]; height];
    let mut rng = rand::thread_rng();

    let mut mines_placed = 0;

    while mines_placed < mine_count {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        if grid[y][x] == 0 {
            grid[y][x] = 1;
            mines_placed += 1;
        }
    }

    grid
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    println!("\x1b[1m  A B C D E F G H\x1b[0m");

    for y in 0..grid.len() {
        print!("\x1b[1m{} \x1b[0m", y+1);
        for x in 0..grid[y].len() {
            print!("{} ", grid[y][x]);
        }
        println!();
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