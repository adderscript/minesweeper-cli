use rand::Rng;

pub struct Grid {
    mine_map: Vec<Vec<i32>>,

    width: usize,
    height: usize,
    mine_count: usize,
}

impl Grid {
    pub fn new(grid_width: usize, grid_height: usize, mine_count: usize) -> Self {
        let mut mine_map = Grid::generate_mine_map(grid_width, grid_height, mine_count);

        Self {
            mine_map: mine_map,

            width: grid_width,
            height: grid_height,
            mine_count: mine_count,
        }
    }

    pub fn print(&self) {
        println!("\x1b[1m  A B C D E F G H\x1b[0m");

        for y in 0..self.height {
            print!("\x1b[1m{} \x1b[0m", y+1);
            for x in 0..self.width {
                print!("{} ", self.mine_map[y][x]);
            }
            println!();
        }
    }

    pub fn get_tile(&mut self, x: usize, y: usize) -> i32 {
        self.mine_map[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, value: i32) {
        self.mine_map[y][x] = value;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn generate_mine_map(width: usize, height: usize, mine_count: usize) -> Vec<Vec<i32>> {
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
}