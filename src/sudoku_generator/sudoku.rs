use rand::prelude::*;

use super::{
    constants::{Grid, Row, GRID_SIZE},
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

    pub fn empty(&mut self) {
        self.grid = Default::default()
    }

    pub fn generate(&mut self) {
        self.empty();

        let mut rng = thread_rng();

        let mut row = 0;
        while row < GRID_SIZE {
            let mut numbers: Vec<u8> = (1..=(GRID_SIZE as u8)).collect();
            let mut col = 0;
            let mut it = 0;
            while col < GRID_SIZE {
                it += 1;
                if it > 50 {
                    self.grid[row] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
                    row -= 1;
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
