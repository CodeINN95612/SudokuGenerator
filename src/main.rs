use sudoku_generator::{sudoku::Sudoku, sudoku_validator::SudokuValidator};

mod sudoku_generator;

fn main() {
    let mut sudoku = Sudoku::new();
    for _ in 0..1 {
        sudoku.generate();
        sudoku.print();
        println!("Is Valid: {}", SudokuValidator::is_valid(&mut sudoku.grid));
    }
}
