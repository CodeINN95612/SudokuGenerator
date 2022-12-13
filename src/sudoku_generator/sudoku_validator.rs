use super::constants::{Grid, GRID_SIZE};

pub struct SudokuValidator {}

impl SudokuValidator {
    pub fn is_number_valid(number: u8, row: usize, col: usize, grid: &Grid) -> bool {
        if number == 0 {
            return false;
        }

        //Validate Row
        if grid[row].contains(&number) {
            return false;
        }

        //Validate Col
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if j != col {
                    continue;
                }
                if grid[i][j] == number {
                    return false;
                }
            }
        }

        //Validate Block
        let pos_row = if row < 3 {
            0
        } else if row < 6 {
            3
        } else {
            6
        };
        let pos_col = if col < 3 {
            0
        } else if col < 6 {
            3
        } else {
            6
        };

        let mut block: Vec<u8> = Vec::new();
        for i in pos_row..pos_row + 3 {
            for j in pos_col..pos_col + 3 {
                block.push(grid[i][j]);
            }
        }
        if block.contains(&number) {
            return false;
        }

        return true;
    }

    pub fn is_valid(grid: &Grid) -> bool {
        for row in grid {
            if !Self::is_group_valid(&row.clone().to_vec()) {
                return false;
            }
        }

        for col in 0..GRID_SIZE {
            let mut column = Vec::new();
            for row in 0..GRID_SIZE {
                column.push(grid[row][col]);
            }
            if !Self::is_group_valid(&column) {
                return false;
            }
        }

        for i in [0, 3, 6] {
            for j in [0, 3, 6] {
                let mut block: Vec<u8> = Vec::new();
                for i in i..(i + 3) {
                    for j in j..(j + 3) {
                        block.push(grid[i][j]);
                    }
                }

                if !Self::is_group_valid(&block) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_group_valid(group: &Vec<u8>) -> bool {
        let mut numbers: Vec<u8> = (1..=(GRID_SIZE as u8)).collect();

        for num in group {
            numbers = numbers.into_iter().filter(|n| n != num).collect();
        }

        numbers.is_empty()
    }
}
