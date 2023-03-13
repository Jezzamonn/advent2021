const DELTAS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SearchState {
    pub x: i32,
    pub y: i32,
    pub dist: u64,
}

impl SearchState {
    pub fn new(x: i32, y: i32, dist: u64) -> Self {
        Self { x, y, dist }
    }

    pub fn neighbors(&self, grid: &Vec<Vec<i32>>) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for (dx, dy) in DELTAS.iter() {
            let x = self.x + dx;
            let y = self.y + dy;
            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }
            let new_dist = self.dist + grid[y as usize][x as usize] as u64;
            neighbors.push(Self::new(x, y, new_dist));
        }
        neighbors
    }

    // A* search heuristic
    pub fn search_score(&self, dest: (i32, i32)) -> u64 {
        let (dx, dy) = (dest.0 - self.x, dest.1 - self.y);
        self.dist + (dx.abs() + dy.abs()) as u64
    }
}