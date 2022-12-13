use rand::prelude::*;

use super::{
    constants::{Grid, GRID_SIZE},
    sudoku_validator::SudokuValidator,
};

pub struct Sudoku {
    pub grid: Grid,
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.grid = Default::default()
    }

    pub fn fill(&mut self) {
        self.clear();

        let mut rng = thread_rng();

        let mut row = 0;
        while row < GRID_SIZE {
            let mut numbers: Vec<u8> = (1..=(GRID_SIZE as u8)).collect();
            let mut col = 0;
            let mut it = 0;
            while col < GRID_SIZE {
                it += 1;
                if it > 50 {
                    self.grid[row] = Default::default();
                    row -= 1;
                    if row > 1 {
                        self.grid[row] = Default::default();
                        row -= 1;
                    }
                    break;
                }

                let (index, num) = numbers.iter().enumerate().choose(&mut rng).unwrap();

                if !SudokuValidator::is_number_valid(*num, row, col, &self.grid) {
                    continue;
                }

                self.grid[row][col] = *num;
                numbers.remove(index);
                col += 1;
            }
            row += 1;
        }
    }

    pub fn remove(&mut self) {
        let mut rng = thread_rng();

        for _ in 0..17 {
            let (mut row, _) = (0..9).enumerate().choose(&mut rng).unwrap();
            let (mut col, _) = (0..9).enumerate().choose(&mut rng).unwrap();
            while self.grid[row][col] == 0 {
                (row, _) = (0..9).enumerate().choose(&mut rng).unwrap();
                (col, _) = (0..9).enumerate().choose(&mut rng).unwrap();
            }
            self.grid[row][col] = 0;
        }
    }

    pub fn print(&self) {
        println!("======SUD OKU======");
        for row in self.grid {
            print!("|");
            for cell in row {
                if cell == 0 {
                    print!(" |");
                } else {
                    print!("{}|", cell);
                }
            }
            println!();
        }
        println!("===================")
    }
}
