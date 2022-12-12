use super::{
    constants::{Grid, GRID_SIZE},
    sudoku_validator::SudokuValidator,
};

pub struct Sudoku {
    grid: Grid,
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    pub fn generate(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                self.grid[i][j] = if SudokuValidator::validate_row(0, &self.grid) {
                    i as u8
                } else {
                    0
                };
            }
        }
    }

    pub fn print(&self) {
        println!("======SUDOKU======");
        for row in self.grid {
            print!("|");
            for cell in row {
                print!("{cell}|");
            }
            println!();
        }
        println!("================")
    }
}
