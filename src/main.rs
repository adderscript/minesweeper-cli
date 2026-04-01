use rand::Rng;
use std::{io, process::Command};

fn main() {
    let grid_width: usize = 8;
    let grid_height: usize = 8;
    let mine_count: usize = 5;

    let grid = generate_grid(grid_width, grid_height, mine_count);
    
    // gameloop
    let mut input = String::new();
    loop {
        Command::new("clear").status().unwrap();
        print_grid(&grid);

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
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