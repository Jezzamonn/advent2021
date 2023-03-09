#[derive(Debug)]
pub struct Grid{
    v: Vec<Vec<u32>>,
    pub w: usize,
    pub h: usize,
}

impl Grid {
    fn new(v: Vec<Vec<u32>>) -> Self {
        let w = v[0].len();
        let h = v.len();
        Self {v, w, h}
    }

    pub fn from_string(string: &str) -> Self {
        Self::new(
            string
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect()
        )
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h
    }

    pub fn get(&self, x: i32, y: i32) -> u32 {
        if !self.in_bounds(x, y) {
            return u32::MAX
        }
        self.v[y as usize][x as usize]
    }
}