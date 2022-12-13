use sudoku_generator::sudoku::Sudoku;

mod sudoku_generator;

fn main() {
    let mut sudoku = Sudoku::new();
    for _ in 0..10 {
        sudoku.fill();
        sudoku.remove();
        sudoku.print();
    }
}
