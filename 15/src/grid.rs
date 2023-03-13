#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<u8>>,
    // Multiplies the size of the grid.
    pub multiple: u8,
}

impl Grid {
    pub fn from_string(s: &str, multiple: u8) -> Self {
        let grid = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        Self { grid, multiple }
    }

    pub fn base_width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn base_height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.base_width() * self.multiple as usize
    }

    pub fn height(&self) -> usize {
        self.base_height() * self.multiple as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }

    pub fn dest(&self) -> (i32, i32) {
        (self.width() as i32 - 1, self.height() as i32 - 1)
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        let mod_y = y as usize % self.base_height();
        let mod_x = x as usize % self.base_width();
        let extra = y as u8 / self.base_height() as u8 + x as u8 / self.base_width() as u8;
        (self.grid[mod_y][mod_x] + extra - 1) % 9 + 1
    }
}

