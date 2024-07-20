const GRID_SIZE: usize = 100;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(self) -> bool {
        self == Self::Alive
    }
}

pub struct Automaton {
    pub grid: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Default for Automaton {
    fn default() -> Self {
        let mut grid = [[Cell::Dead; GRID_SIZE]; GRID_SIZE];
        grid[50][50] = Cell::Alive;
        grid[50][51] = Cell::Alive;
        grid[49][51] = Cell::Alive;
        grid[50][52] = Cell::Alive;
        grid[51][52] = Cell::Alive;

        Self { grid }
    }
}

impl Automaton {
    fn count_neighbours(&self, i: usize, j: usize) -> usize {
        let mut neighbours: usize = 0;
        let grid = self.grid;

        if i < (GRID_SIZE - 1) && grid[i + 1][j].is_alive() {
            neighbours += 1;
        }

        if i > 0 && grid[i - 1][j].is_alive() {
            neighbours += 1;
        }

        if j < (GRID_SIZE - 1) && grid[i][j + 1].is_alive() {
            neighbours += 1
        }

        if j > 0 && grid[i][j - 1].is_alive() {
            neighbours += 1
        }

        if i < (GRID_SIZE - 1) && j < (GRID_SIZE - 1) && grid[i + 1][j + 1].is_alive() {
            neighbours += 1
        }

        if i < (GRID_SIZE - 1) && j > 0 && grid[i + 1][j - 1].is_alive() {
            neighbours += 1
        }

        if i > 0 && j < (GRID_SIZE - 1) && grid[i - 1][j + 1].is_alive() {
            neighbours += 1
        }

        if i > 0 && j > 0 && grid[i - 1][j - 1].is_alive() {
            neighbours += 1
        }

        neighbours
    }

    pub fn update(&mut self) {
        let mut new_grid = [[Cell::Dead; GRID_SIZE]; GRID_SIZE];
        for i in 0..self.grid.len() {
            let line = self.grid[i];

            for j in 0..line.len() {
                let neighbours = self.count_neighbours(i, j);

                match self.grid[i][j] {
                    Cell::Alive => {
                        if neighbours == 2 || neighbours == 3 {
                            new_grid[i][j] = Cell::Alive;
                        } else {
                            new_grid[i][j] = Cell::Dead;
                        }
                    }
                    Cell::Dead => {
                        if neighbours == 3 {
                            new_grid[i][j] = Cell::Alive;
                        } else {
                            new_grid[i][j] = Cell::Dead;
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }
}
