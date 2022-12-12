use sudoku_generator::sudoku::Sudoku;

mod sudoku_generator;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.generate();
    sudoku.print();
}
